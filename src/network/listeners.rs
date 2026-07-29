// 引入 pnet 提供的底层网络接口和通道功能
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;

pub fn start_listener() {
    // 获取当前所有网卡
    // 在 C++ 中这需要调用繁琐的 GetAdaptersAddresses，Rust 封装成了 Vec<NetworkInterface>
    let interfaces : Vec<NetworkInterface> = datalink::interfaces();
    // stack
    // ┌──────────────────────┐
    // │ interfaces            │
    // │ Vec<NetworkInterface> │
    // └──────────────────────┘
    //          |
    //          |
    //          v
    // heap:
    //  [
    //    NetworkInterface,
    //    NetworkInterface,
    //    NetworkInterface,
    //  ]
    // 这里发生了所有权的转移
    // for iface in interfaces {
    // interfaces想后续使用，应该使用借用
    for iface in &interfaces {
        println!("name: {}", iface.name);
        println!("desc: {}", iface.description);
        println!("index: {}", iface.index);
        println!("mac: {:#?}", iface.mac);
        println!("all ips: {:#?}", iface.ips);
        println!("flags: {}", iface.flags);
        println!("enabled: {}", iface.is_up());
    }

    // iface为&interface
    let interface = interfaces
        .into_iter()
        .find(|iface| {
                if iface.is_loopback() {
                    return false;
                }

                if iface.ips.is_empty() {
                    return false;
                }

                iface.ips.iter().any(|ip_net| {
                    let ip_str = ip_net.ip().to_string();
                    ip_str != "0.0.0.0" && !ip_str.starts_with("169.254.") && !ip_str.starts_with("127.")
                })
            })
        .expect("致命错误: 没有找到合适的活跃网卡！请检查网络或是否安装了 Npcap。");

    println!("========================================");
    println!("🚀 成功锁定监听网卡: {}", interface.name);
    println!("📍 网卡 MAC 地址: {:?}", interface.mac);
    println!("desc: {}", interface.description);
    println!("index: {}", interface.index);
    println!("mac: {:#?}", interface.mac);
    println!("all ips: {:#?}", interface.ips);
    println!("========================================");

    // 在这个网卡上打开一个数据链路层的通道
    // 相当于C++中创建一个Raw Socket并绑定。
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("系统环境不支持以太网数据链路层抓包！"),
        Err(e) => panic!("创建网卡通道失败: {} (🚨 警告: 在 Windows 上请务必以管理员权限运行！)", e),
    };

    println!("📡 开始在混杂模式下监听以太网帧 (按 Ctrl+C 退出)...");

    loop {
        match rx.next() {
            Ok(packet) => {
                // 这里的 packet 类型是 &[u8]，即“字节切片”。
                // 它是零拷贝 (Zero-Copy) 的！它的内存就在网卡驱动的缓冲区里，我们只是借用 (Borrow) 了它。
                crate::network::parser::parse_raw_packet(packet);
            },
            Err(e) => {
                // 如果发生瞬时错误，比如网卡短暂离线，不应该让程序崩溃，打印错误即可
                eprintln!("读取网络包发生异常: {}", e);
            }
        }
    }
}
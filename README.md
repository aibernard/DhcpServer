# DhcpServer
DhcpServer test tool for Windows






# pnet
## pnet::datalink::NetworkInterface
NetworkInterface是一个结构体，用来描述主机上的一块网卡。NetworkInterface包含了网卡的名称、索引、MAC地址、IP地址等信息。通过NetworkInterface，我们可以获取主机上所有网卡的信息，并对其进行操作。
```Rust
pub struct NetworkInterface {
    pub name: String,              // 接口名字
    pub description: String,       // 描述（Windows 上比较有用）
    pub index: u32,                // 操作系统分配的索引号
    pub mac: Option<MacAddr>,      // MAC 地址（可能没有）
    pub ips: Vec<IpNetwork>,       // 绑定的 IP 地址列表
    pub flags: u32,                // 操作系统标志位
}
```
NetworkInterface提供了很多便捷的判断方法，底层其实是在检查flags字段。
```Rust
impl NetworkInterface {
    pub fn is_up(&self) -> bool;               // 网卡是否启用（启用了才可能收发包）
    pub fn is_loopback(&self) -> bool;         // 是不是回环接口（127.0.0.1 那个）
    pub fn is_broadcast(&self) -> bool;        // 是否支持广播
    pub fn is_point_to_point(&self) -> bool;   // 是否点对点（比如 VPN、PPP）
    pub fn is_multicast(&self) -> bool;        // 是否支持组播
    pub fn is_running(&self) -> bool;          // 是否正在运行
    // Linux 特有：
    pub fn is_lower_up(&self) -> bool;         // 物理链路是否 up（网线插好了）
    pub fn is_dormant(&self) -> bool;          // 是否处于休眠状态
}
```

## Channel
Channel是pnet提供的一个抽象，用来表示网络接口的发送和接收,是数据链路层的“管道”。Channel分为两种类型：DataLinkSender和DataLinkReceiver，分别用于发送和接收数据包。通过Channel，我们可以在指定的网络接口上发送和接收原始数据包。
Channel 是一个枚举，代表“在数据链路层（第二层）收发数据包的通道”。
可以把它想象成：
你打开了一根直接连到网卡的“水管”，
一边可以往里灌原始以太网帧（发送），
一边可以从里面捞原始以太网帧（接收）。
```Rust
#[non_exhaustive] // 这个枚举可能会在未来添加新的变体，所以匹配时必须写_通配分支
pub enum Channel {
    Ethernet(
        Box<dyn DataLinkSender>,    // 发送端
        Box<dyn DataLinkReceiver>,  // 接收端
    ),
}
```
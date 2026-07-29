/// 文件：src/network/parser.rs
/// 引入 pnet 内置的各种协议解析包
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;

/// 整个网络解析的统一入口
/// 注意入参的生命周期：raw_data 是底层网卡缓冲区的借用，我们全程不会进行 clone
pub fn parse_raw_packet(raw_data: &[u8]) {
    // 尝试将字节流解析为以太网帧
    let eth_packet = match EthernetPacket::new(raw_data) {
        Some(packet) => packet,
        None => return, // 如果包太小连以太网头都凑不够，直接丢弃
    };

    // 根据以太网的EtherType 字段，判断下一层协议
    match eth_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            parse_ipv4(eth_packet.payload());
        }
        _ => {
            println!("ignore ipv6");
        }
    }
}

fn parse_ipv4(payload: &[u8]) {
    if let Some(ipv4_packet) = Ipv4Packet::new(payload) {
        if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
            parse_udp(ipv4_packet.payload());
        }
    }
}

fn parse_udp(payload: &[u8]) {
    if let Some(udp_packet) = UdpPacket::new(payload) {
        let dest_port = udp_packet.get_destination();
        let src_port = udp_packet.get_source();

        // 客户端发给 DHCP 服务器的包，目的端口一定是 67
        // (标准 DHCPv4: 客户端端口 68 -> 服务端端口 67)
        if dest_port == 67 {
            println!("==================================================");
            println!("🎉 [命中!] 捕获到目标端口 67 的 DHCP 报文！");
            println!("来源端口: {}, 目标端口: {}", src_port, dest_port);
            
            let dhcp_payload = udp_packet.payload();
            println!("DHCP 载荷大小: {} bytes", dhcp_payload.len());
            
            // 打印前 16 个字节的十六进制，看看 DHCP 报文长什么样
            let hex_preview = dhcp_payload.iter().take(16)
                .map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");
            println!("载荷预览 (Hex): {} ...", hex_preview);
            println!("==================================================\n");
        } else {
            // println!("==================================================");
            // println!("[未命中]来源端口: {}, 目标端口: {}", src_port, dest_port);
            // let dhcp_payload = udp_packet.payload();
            // println!("UDP 载荷大小: {} bytes", dhcp_payload.len());
            // println!("==================================================\n");
        }
    }
}
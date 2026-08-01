use std::net::Ipv4Addr;
use std::convert::TryInto;

// 生命周期'a绑定到底层的&[u8]数据，保证只要Packet活着，底层数据就不会释放
#[derive(Debug)]
pub struct DhcpPacket<'a> {
    pub op: u8,
    pub htype: u8,
    pub hlen: u8,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: Ipv4Addr, // client IP
    pub yiaddr: Ipv4Addr, // Your IP
    pub chaddr: &'a [u8], // Client MAC Address (截取前 hlen 长度的切片)
    pub options: &'a [u8], // 变长的 Options 区域 (跳过 236 字节的固定头部和 Magic Cookie)
}

impl<'a> DhcpPacket<'a> {
    pub fn parse(payload: &'a [u8]) -> Option<Self> {
        // DHCP 固定头部最小 236 字节 + 4字节 Magic Cookie
        if payload.len() < 240 {
            return None; 
        }

        // 验证 DHCP Magic Cookie (99, 130, 83, 99)，证明这不是一个废包
        let cookie = &payload[236..240];
        if cookie != [99, 130, 83, 99] {
            return None;
        }

        Some(DhcpPacket {
            op: payload[0],
            htype: payload[1],
            hlen: payload[2],
            hops: payload[3],
            // Rust 极其安全的字节序转换！绝不使用强制指针转换
            // [4..8] 取出 4 个字节，try_into() 转为 [u8; 4] 数组，然后按大端 (网络字节序) 解析
            xid: u32::from_be_bytes(payload[4..8].try_into().unwrap()),
            secs: u16::from_be_bytes(payload[8..10].try_into().unwrap()),
            flags: u16::from_be_bytes(payload[10..12].try_into().unwrap()),
            ciaddr: Ipv4Addr::new(payload[12], payload[13], payload[14], payload[15]),
            yiaddr: Ipv4Addr::new(payload[16], payload[17], payload[18], payload[19]),
            // 客户端 MAC 地址，根据 hlen (通常是 6，即以太网 MAC) 截取切片
            chaddr: &payload[28..(28 + payload[2] as usize)],
            // 剩下的全部归为 Options
            options: &payload[240..],
        })
    }
}
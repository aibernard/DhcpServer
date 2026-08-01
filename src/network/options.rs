use std::net::Ipv4Addr;

pub enum DhcpOption<'a> {
    MessageType(u8),           // Option 53 (1: Discover, 3: Request)
    RequestedIp(Ipv4Addr),     // Option 50
    ClientIdentifier(&'a [u8]),// Option 61 (RFC 4361 核心)
    Unknown(u8, &'a [u8]),
}

const OPTION_SUBNET_MASK: u8                    = 1;
const OPTION_ROUTER: u8                         = 3;
const OPTION_DNS_LIST: u8                       = 6;
const OPTION_HOST_NAME:u8                       = 12;
const OPTION_DOMAIN_NAME: u8                    = 15;
const OPTION_PERFORM_ROUTER_DISCOVER: u8        = 31;
const OPTION_STATIC_ROUTE: u8                   = 33;
const OPTION_VENDOR_SPEC_INFO: u8               = 43;
const OPTION_NETBIOS_NAME_SERVER: u8            = 44;
const OPTION_NETBIOS_NODE_TYPE: u8              = 46;
const OPTION_NETBIOS_SCOPE: u8                  = 47;
const OPTION_REQUEST_IP_ADDRESS: u8             = 50;
const OPTION_IP_ADDRESS_LEASE: u8               = 51;
const OPTION_DHCP_MESSAGE_TYPE: u8              = 53;
const OPTION_SERVER_IDENTIFIER: u8              = 54;
const OPTION_PARAMETER_REQUEST_LIST: u8         = 55;
const OPTION_MESSAGE: u8                        = 56;
const OPTION_CLIENT_IDENTIFIER: u8              = 61;
const OPTION_CFQDN: u8                          = 81;
const OPTION_VENDOR_CLASS_IDENTIFIER: u8        = 60;
const OPTION_DOMAIN_SEARCH: u8                  = 119;
const OPTION_CLASS_STATIC_ROUTE: u8             = 121;
const OPTION_END: u8                            = 255;


pub fn parse_options<'a>(mut options_data: &'a [u8]) -> Vec<DhcpOption<'a>> {
    let mut options = Vec::new();

    while !options_data.is_empty() {
        let opt_code = options_data[0];
        
        if opt_code == 255 { break; } // End Option (结束符)
        if opt_code == 0 {            // Pad Option (填充符)，长度固定为1
            options_data = &options_data[1..]; 
            continue; 
        }

        // 安全检查：防止只有 code 没有 length
        if options_data.len() < 2 { break; } 
        let opt_len = options_data[1] as usize;

        // 越界
        if options_data.len() < 2 + opt_len { break; }

        let opt_val = &options_data[2..2 + opt_len];

        match opt_code {
            53 if opt_len == 1 => options.push(DhcpOption::MessageType(opt_val[0])),
            50 if opt_len == 4 => {
                let ip = Ipv4Addr::new(opt_val[0], opt_val[1], opt_val[2], opt_val[3]);
                options.push(DhcpOption::RequestedIp(ip));
            }
            61 => options.push(DhcpOption::ClientIdentifier(opt_val)),
            _ => options.push(DhcpOption::Unknown(opt_code, opt_val)),
        }

        options_data = &options_data[2 + opt_len..];
    }
    options
}
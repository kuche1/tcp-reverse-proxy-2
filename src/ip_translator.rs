use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

pub struct IpTranslator {
    ip_first_to_use: u32,
    ip_last_used: u32,
    ip_last_available: u32,

    ip_map: HashMap<IpAddr, Ipv4Addr>,
}

impl IpTranslator {
    pub fn new() -> Self {
        let ip = (127 << 24) | (0 << 16) | (0 << 8) | (2 << 0);

        IpTranslator {
            ip_first_to_use: ip,
            ip_last_used: ip - 1,
            ip_last_available: (127 << 24) | (255 << 16) | (255 << 8) | (254 << 0), // 127.255.255.255 is broadcast

            ip_map: HashMap::new(),
        }
    }

    fn get_next_ip(&mut self) -> Ipv4Addr {
        let ip_u32 = {
            let current = self.ip_last_used + 1;
            if current > self.ip_last_available {
                // actually untested, but it should be fine
                self.ip_first_to_use
            } else {
                current
            }
        };

        self.ip_last_used = ip_u32;

        Ipv4Addr::from(ip_u32)
    }

    pub fn translate(&mut self, original_ip: IpAddr) -> Ipv4Addr {
        match self.ip_map.get(&original_ip) {
            Some(v) => return *v,
            None => {}
        };

        let new_ip = self.get_next_ip();

        self.ip_map.insert(original_ip, new_ip);

        new_ip
    }
}

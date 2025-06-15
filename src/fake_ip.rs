use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub struct FakeIpGenerator {
    ip_emergency: u32, // used when out of IPs
    ip_last_used: u32,
    ip_last_available: u32,
}

impl FakeIpGenerator {
    pub fn new() -> Self {
        let ip = (127 << 24) | (0 << 16) | (0 << 8) | (1 << 0);

        FakeIpGenerator {
            ip_emergency: ip,
            ip_last_used: ip,
            ip_last_available: (127 << 24) | (255 << 16) | (255 << 8) | (254 << 0), // 127.255.255.255 is broadcast
        }
    }

    pub fn gen_next(&mut self) -> u32 {
        let current = self.ip_last_used + 1;
        if current > self.ip_last_available {
            // TODO write to error folder
            return self.ip_emergency;
        }

        self.ip_last_used = current;

        current
    }
}

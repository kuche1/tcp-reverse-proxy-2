pub struct FakeIpGenerator {
    ip_first: u32,
    ip_last: u32,
    ip_current: u32,
}

impl FakeIpGenerator {
    pub fn new() -> Self {
        let first = (127 << 24) | (0 << 16) | (0 << 8) | (2 << 0);
        // 127.0.0.1 is also valid, but we'll reserve that

        FakeIpGenerator {
            ip_first: first,
            ip_last: (127 << 24) | (255 << 16) | (255 << 8) | (254 << 0), // 127.255.255.255 is broadcast
            ip_current: first - 1,
        }
    }

    pub fn gen_next(&mut self) -> Option<u32> {
        let current = self.ip_current + 1;
        if current > self.ip_last {
            return None;
        }

        self.ip_current = current;

        Some(current)
    }
}

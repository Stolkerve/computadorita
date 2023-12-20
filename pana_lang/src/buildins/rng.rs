pub struct Rng {
    state: (u64, u64),
}

impl Rng {
    pub fn new(random_seed: u64) -> Self {
        let mut seed = 0;

        seed |= random_seed << 56;
        seed |= random_seed << 48;
        seed |= random_seed << 40;
        seed |= random_seed << 32;
        seed |= random_seed << 24;
        seed |= random_seed << 16;
        seed |= random_seed << 8;
        seed |= random_seed << 0;

        Self {
            state: (
                seed ^ 0xf4dbdf2183dcefb7, // [crc32(b"0"), crc32(b"1")]
                seed ^ 0x1ad5be0d6dd28e9b, // [crc32(b"2"), crc32(b"3")]
            ),
        }
    }

    pub fn rand_u64(&mut self) -> u64 {
        let (mut x, y) = self.state;
        self.state.0 = y;
        x ^= x << 23;
        self.state.1 = x ^ y ^ (x >> 17) ^ (y >> 26);
        self.state.1.wrapping_add(y)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn rand_usize(&mut self) -> usize {
        self.rand_u32() as usize
    }

    pub fn rand_bounded_u64(&mut self, m: u64) -> u64 {
        let threshold = m.wrapping_neg().wrapping_rem(m);
        loop {
            let r = self.rand_u64();
            if r >= threshold {
                return r.wrapping_rem(m);
            }
        }
    }

    #[cfg(target_pointer_width = "32")]
    pub fn rand_bounded_usize(&mut self, m: usize) -> usize {
        self.rand_bounded_u32(m as u32) as usize
    }

    pub fn rand_range_i64(&mut self, a: i64, b: i64) -> i64 {
        a + self.rand_bounded_u64((b - a) as u64) as i64
    }
}

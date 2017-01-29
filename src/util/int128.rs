use std;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[allow(non_camel_case_types)]
pub struct u128 {
    high: u64,
    low: u64,
}

impl u128 {
    pub fn zero() -> u128 {
        u128::from_parts(0, 0)
    }

    pub fn from_parts(high: u64, low: u64) -> u128 {
        u128 {
            high: high,
            low: low,
        }
    }

    pub fn parts(&self) -> (u64, u64) {
        (self.high, self.low)
    }
}

impl std::ops::Add<u128> for u128 {
    type Output = u128;
    fn add(self, rhs: u128) -> u128 {
        let low = self.low + rhs.low;
        let high = self.high + rhs.high +
                   if low < self.low {
            1
        } else {
            0
        };

        u128::from_parts(high, low)
    }
}

impl<'a> std::ops::Add<&'a u128> for u128 {
    type Output = u128;
    fn add(self, rhs: &'a u128) -> u128 {
        let low = self.low + rhs.low;
        let high = self.high + rhs.high +
                   if low < self.low {
            1
        } else {
            0
        };

        u128::from_parts(high, low)
    }
}

impl std::convert::From<u8> for u128 {
    fn from(n: u8) -> u128 {
        u128::from_parts(0, n as u64)
    }
}


impl std::ops::Mul<u128> for u128 {
    type Output = u128;

    fn mul(self, rhs: u128) -> u128 {
        let top: [u64; 4] = [self.high >> 32,
                             self.high & 0xFFFFFFFF,
                             self.low >> 32,
                             self.low & 0xFFFFFFFF];

        let bottom: [u64; 4] = [rhs.high >> 32,
                                rhs.high & 0xFFFFFFFF,
                                rhs.low >> 32,
                                rhs.low & 0xFFFFFFFF];

        let mut rows = [u128::zero(); 16];
        for i in 0..4 {
            for j in 0..4 {
                let shift = i + j;
                let product = top[3 - i] * bottom[3 - j];
                let (high, low) = match shift {
                    0 => (0, product),
                    1 => (product >> 32, product << 32),
                    2 => (product, 0),
                    3 => (product << 32, 0),
                    _ => {
                        if product == 0 {
                            (0, 0)
                        } else {
                            panic!("Overflow on mul {:?} {:?} ({} {})", self, rhs, i, j)
                        }
                    }
                };
                rows[j * 4 + i] = u128::from_parts(high, low);
            }
        }

        rows.iter().fold(u128::zero(), std::ops::Add::add)
    }
}

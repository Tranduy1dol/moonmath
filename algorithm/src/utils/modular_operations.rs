#[derive(Debug, Copy, Clone)]
struct Modular(pub i64);

impl Modular {
    pub fn add(self, a: i64, b: i64) -> u64 {
        (((a % self.0 + b % self.0) % self.0 + self.0) % self.0) as u64
    }

    pub fn sub(self, a: i64, b: i64) -> u64 {
        (((a % self.0 - b % self.0) % self.0 + self.0) % self.0) as u64
    }

    pub fn mul(self, a: i64, b: i64) -> u64 {
        (((a % self.0 * b % self.0) % self.0 + self.0) % self.0) as u64
    }

    pub fn inv(self, a: i64) -> Option<u64> {
        for i in 1..self.0 {
            if (a * i as i64) % self.0 as i64 == 1 {
                return Some(i as u64);
            }
        }
        None
    }

    pub fn div(self, a: i64, b: i64) -> Option<u64> {
        if let Some(inv_b) = Self::inv(self, b) {
            Some(Self::mul(self, a, inv_b as i64))
        } else {
            None
        }
    }
}

mod test {
    use crate::utils::modular_operations::Modular;

    #[test]
    fn test_modular() {
        let modular_5 = Modular(5);
        assert_eq!(modular_5.inv(2).unwrap(), 3);
        assert_eq!(modular_5.add(10, 4), 4);
        assert_eq!(modular_5.sub(10, 6), 4);
        assert_eq!(modular_5.mul(10, 4), 0);
        assert_eq!(modular_5.div(8, 3).unwrap(), 1);
    }
}

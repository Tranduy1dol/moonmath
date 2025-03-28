use crate::extended_gcd::extended_gcd;

#[derive(Debug, Copy, Clone)]
pub struct Modular(pub i64);

impl Modular {
    /// Adds two numbers in the given modular field.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number.
    /// * `b` - The second number.
    ///
    /// # Returns
    ///
    /// The result of (a + b) % modulus.
    pub fn add(&self, a: i64, b: i64) -> i64 {
        ((a + b) % self.0 + self.0) % self.0
    }

    /// Subtracts two numbers in the given modular field.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number.
    /// * `b` - The second number.
    ///
    /// # Returns
    ///
    /// The result of (a - b) % modulus.
    pub fn sub(&self, a: i64, b: i64) -> i64 {
        self.add(a, self.neg(b))
    }

    /// Multiplies two numbers in the given modular field.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number.
    /// * `b` - The second number.
    ///
    /// # Returns
    ///
    /// The result of (a * b) % modulus.
    pub fn mul(&self, a: i64, b: i64) -> i64 {
        ((a * b) % self.0 + self.0) % self.0
    }

    /// Negates a number in the given modular field.
    ///
    /// # Arguments
    ///
    /// * `a` - The number to be negated.
    ///
    /// # Returns
    ///
    /// The result of -a % modulus.
    pub fn neg(&self, a: i64) -> i64 {
        (((self.0 - a) % self.0) + self.0) % self.0
    }

    /// Computes the modular inverse of a number.
    ///
    /// # Arguments
    ///
    /// * `a` - The number to be inverted.
    ///
    /// # Returns
    ///
    /// The modular inverse of `a`.
    pub fn inv(&self, a: i64) -> i64 {
        let (inv, _, _) = extended_gcd(a, self.0);
        self.add(inv, self.0)
    }

    /// Divides two numbers in the given modular field.
    ///
    /// # Arguments
    ///
    /// * `a` - The dividend.
    /// * `b` - The divisor.
    ///
    /// # Returns
    ///
    /// The result of (a / b) % modulus.
    pub fn div(&self, a: i64, b: i64) -> anyhow::Result<i64> {
        if b == 0 {
            anyhow::bail!("Division by zero");
        } else {
            Ok(self.mul(a, self.inv(b)))
        }
    }
}

mod test {
    use crate::utils::modular_operations::Modular;

    #[test]
    fn test_modular() {
        let modular_5 = Modular(5);
        assert_eq!(modular_5.inv(2), 3);
        assert_eq!(modular_5.add(10, 4), 4);
        assert_eq!(modular_5.sub(0, 6), 4);
        assert_eq!(modular_5.mul(10, 4), 0);
        assert_eq!(modular_5.div(8, 3).unwrap(), 1);
    }
}

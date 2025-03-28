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
    /// Computes the modular sum of two integers.
    ///
    /// Returns the result of (a + b) modulo the modulus stored in the Modular instance,
    /// ensuring that the result is normalized within the range [0, modulus).
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithm::utils::modular_operations::Modular;
    ///
    /// let modular = Modular(5);
    /// assert_eq!(modular.add(3, 4), 2); // (3 + 4) mod 5 = 2
    ///
    /// // Negative values are normalized correctly:
    /// assert_eq!(modular.add(-3, 2), 4); // (-3 + 2) mod 5 = 4
    /// ```    pub fn add(&self, a: i64, b: i64) -> i64 {
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
    /// Computes the modular subtraction of `b` from `a`.
    ///
    /// This method returns the result of `(a - b) mod modulus` by adding `a` to the modular negation of `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// let modular = Modular(5);
    /// // (3 - 4) mod 5 yields 4 because -1 mod 5 equals 4.
    /// assert_eq!(modular.sub(3, 4), 4);
    /// ```    pub fn sub(&self, a: i64, b: i64) -> i64 {
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
    /// Computes the product of two integers modulo the instance's modulus.
    ///
    /// This method returns the result of `((a * b) % modulus + modulus) % modulus`, ensuring
    /// that the result is non-negative even if `a * b` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// // For a modulus of 7: (3 * 4) mod 7 equals 5.
    /// let modulus = Modular(7);
    /// assert_eq!(modulus.mul(3, 4), 5);
    /// ```    pub fn mul(&self, a: i64, b: i64) -> i64 {
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
    /// Returns the modular negation of a given number.
    /// 
    /// Computes the additive inverse of `a` in the modular field defined by the struct's modulus.
    /// The result is always a non-negative integer in the range `[0, modulus)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let modular = Modular(5);
    /// // The additive inverse of 3 modulo 5 is 2 because (3 + 2) mod 5 equals 0.
    /// assert_eq!(modular.neg(3), 2);
    /// ```    pub fn neg(&self, a: i64) -> i64 {
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
    /// Computes the modular multiplicative inverse of `a` modulo the instance's modulus.
    ///
    /// Given an integer `a`, this method returns the number `x` such that `(a * x) % m == 1`, where
    /// `m` is the modulus stored in this instance. The calculation leverages the extended Euclidean
    /// algorithm and normalizes the result to ensure it falls within the correct range.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithm::utils::modular_operations::Modular;
    ///
    /// let modular = Modular(7);
    /// let inv = modular.inv(3);
    /// assert_eq!((3 * inv) % 7, 1);
    /// ```    pub fn inv(&self, a: i64) -> i64 {
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
    /// Divides `a` by `b` under modular arithmetic with the modulus defined by the struct.
    ///
    /// This method computes the result as `(a * inv(b)) % modulus`, where `inv(b)` is the modular inverse of `b`.
    /// If `b` is zero, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// // Assume the Modular struct is available from the appropriate module.
    /// let modulo = Modular(5);
    ///
    /// // Perform modular division (mod 5)
    /// let result = modulo.div(3, 2)?;
    /// assert_eq!(result, modulo.mul(3, modulo.inv(2)));
    ///
    /// // Division by zero yields an error.
    /// assert!(modulo.div(3, 0).is_err());
    /// # Ok::<(), anyhow::Error>(())
    /// ```    pub fn div(&self, a: i64, b: i64) -> anyhow::Result<i64> {
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

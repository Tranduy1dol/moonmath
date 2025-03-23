/// Performs cyclic group exponentiation.
///
/// # Arguments
///
/// * `g` - The base of the exponentiation.
/// * `x` - The exponent.
/// * `n` - The modulus.
///
/// # Returns
///
/// The result of (g ^ x) % n.
pub fn cge(g: u64, mut x: u64, n: u64) -> u64 {
    let mut result = 1;
    let mut base = g;
    while x > 0 {
        if x & 1 == 1 {
            result = (result * base) % n;
        }
        base = (base * base) % n;
        x >>= 1;
    }
    result
}

/// Performs exponentiation by squaring method.
///
/// # Arguments
///
/// * `g` - The base of the exponentiation.
/// * `x` - The exponent.
/// * `n` - The modulus.
///
/// # Returns
///
/// The result of (g * x) % n.
pub fn esm(g: u64, mut x: u64, n: u64) -> u64 {
    let mut result = 0;
    let mut base = g;
    while x > 0 {
        if x & 1 == 1 {
            result = (result + base) % n;
        }
        base = (base * 2) % n;
        x >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cge() {
        assert_eq!(cge(3, 5, 13), 9);
        assert_eq!(cge(5, 3, 23), 10);
    }

    #[test]
    fn test_esm() {
        assert_eq!(esm(3, 10, 13), 4);
        assert_eq!(esm(7, 10, 23), 1);
    }
}

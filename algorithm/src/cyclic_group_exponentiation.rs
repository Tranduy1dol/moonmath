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
/// Computes (g^x) mod n using exponentiation by squaring.
///
/// This function efficiently computes the modular exponentiation of a base `g` raised to an exponent `x`
/// modulo `n`. It uses the exponentiation by squaring technique to minimize the number of multiplications,
/// making it suitable for large exponent values.
///
/// # Examples
///
/// ```
/// let result = cge(2, 10, 1000);
/// // 2^10 = 1024, and 1024 mod 1000 is 24.
/// assert_eq!(result, 24);
/// ```pub fn cge(g: u64, mut x: u64, n: u64) -> u64 {
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
/// Computes the product of `g` and `x` modulo `n` using an iterative doubling algorithm.
///
/// This function processes each bit in `x` to accumulate the modular product. If a bit is set,
/// the current value of `g` is added to the result modulo `n`. The value of `g` is then doubled
/// (with modulus `n`) for the next bit, ensuring that the operation is performed safely even for large values.
///
/// # Examples
///
/// ```
/// let result = esm(5, 3, 7);
/// assert_eq!(result, (5 * 3) % 7);
/// ```pub fn esm(g: u64, mut x: u64, n: u64) -> u64 {
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

/// Computes the extended Euclidean algorithm.
///
/// # Arguments
///
/// * `x` - The first integer.
/// * `y` - The second integer.
///
/// # Returns
///
/// A tuple containing the greatest common divisor and the coefficients of Bézout's identity.
pub fn extended_gcd(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_s, old_t, old_r)
}

mod test {
    use crate::extended_gcd::extended_gcd;

    #[test]
    fn test_extended_gcd() {
        assert_eq!(extended_gcd(12, 5), (-2, 5, 1));
        assert_eq!(extended_gcd(45, 10), (1, -4, 5));
    }
}

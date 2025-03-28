/// Computes the extended Euclidean algorithm.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// Computes the extended greatest common divisor (GCD) of two integers.
/// 
/// Given integers `x` and `y`, returns a tuple `(a, b, g)` where `a` and `b` are the Bézout coefficients satisfying
/// the equation `a * x + b * y = g`, and `g` is the greatest common divisor of `x` and `y`.
/// 
/// # Examples
/// 
/// ```
/// let (a, b, g) = extended_gcd(12, 5);
/// // Verify Bézout's identity
/// assert_eq!(a * 12 + b * 5, g);
///
/// let (a, b, g) = extended_gcd(45, 10);
/// assert_eq!(a * 45 + b * 10, g);
/// ```pub fn extended_gcd(x: i64, y: i64) -> (i64, i64, i64) {
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

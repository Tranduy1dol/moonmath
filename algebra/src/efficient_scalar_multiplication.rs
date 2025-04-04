/// Performs efficient scalar multiplication by double-and-add method.
///
/// # Arguments
///
/// * `g` - The base of the multiplication.
/// * `x` - The scalar.
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

mod test {
    use super::*;

    #[test]
    fn test_esm() {
        assert_eq!(esm(3, 10, 13), 4);
        assert_eq!(esm(7, 10, 23), 1);
        assert_eq!(esm(0, 10, 13), 0);
        assert_eq!(esm(3, 0, 13), 0);
    }
}

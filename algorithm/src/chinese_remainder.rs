use crate::extended_gcd::extended_gcd;

/// Solves the system of congruences using the Chinese Remainder Theorem.
///
/// # Arguments
///
/// * `a` - A vector of remainders.
/// * `n` - A vector of moduli.
///
/// # Returns
///
/// Solves a system of congruences using the Chinese Remainder Theorem and returns the smallest non-negative solution.
///
/// Given two vectors `a` and `n` representing remainders and moduli respectively, this function computes an integer `x` such that:
/// 
/// - x ≡ a[i] (mod n[i]) for each corresponding element in `a` and `n`
/// - x is the smallest non-negative solution modulo the product of all elements in `n`
///
/// # Errors
///
/// Returns an error if the lengths of `a` and `n` do not match.
///
/// # Examples
///
/// ```
/// let a = vec![2, 3, 2];
/// let n = vec![3, 5, 7];
/// let solution = chinese_remainder_solver(a, n).unwrap();
/// assert_eq!(solution, 23);
/// ```pub fn chinese_remainder_solver(a: Vec<i64>, n: Vec<i64>) -> anyhow::Result<i64> {
    if n.len() != a.len() {
        anyhow::bail!("Length mismatch");
    }

    let n_product = n.iter().product::<i64>();
    let mut x_prime = 0;

    for j in 0..a.len() {
        let a_j = a.get(j).unwrap();
        let n_j = n.get(j).unwrap();

        let n_product_j = n_product / n_j;
        let (s_j, _, _) = extended_gcd(n_product_j, *n_j);

        x_prime += a_j * s_j * n_product_j;
    }

    let x = x_prime % n_product;

    Ok(x)
}

mod test {
    use crate::chinese_remainder::chinese_remainder_solver;

    #[test]
    fn test_chinese_remainder_solver() {
        let a: Vec<i64> = vec![4, 1, 3, 0];
        let n: Vec<i64> = vec![7, 3, 5, 11];

        let result = chinese_remainder_solver(a, n).unwrap();
        assert_eq!(result, 88);
    }
}

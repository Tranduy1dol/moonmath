use utils::extended_gcd::extended_gcd;

/// Solves the system of congruences using the Chinese Remainder Theorem.
///
/// # Arguments
///
/// * `a` - A vector of remainders.
/// * `n` - A vector of moduli.
///
/// # Returns
///
/// The smallest non-negative solution to the system of congruences.
pub fn chinese_remainder_solver(a: Vec<i64>, n: Vec<i64>) -> anyhow::Result<i64> {
    if n.len() != a.len() {
        anyhow::bail!("Length mismatch");
    }

    // Check that all moduli are positive
    if n.iter().any(|&x| x <= 0) {
        anyhow::bail!("All moduli must be positive");
    }

    // Check that moduli are pairwise coprime
    for i in 0..n.len() {
        for j in 0..n.len() {
            if i != j {
                let (_, _, gcd) = extended_gcd(n[i], n[j]);
                if gcd != 1 {
                    anyhow::bail!("Moduli must be pairwise coprime");
                }
            }
        }
    }

    let n_product = n.iter().product::<i64>();
    let mut x_prime: i64 = 0;

    for j in 0..a.len() {
        let a_j = a[j];
        let n_j = n[j];

        let n_product_j = n_product / n_j;
        let (s_j, _, _) = extended_gcd(n_product_j, n_j);

        // Use wrapping_add to prevent overflow
        x_prime = x_prime.wrapping_add((a_j * s_j * n_product_j) % n_product);
        x_prime %= n_product;
    }

    let x = x_prime % n_product;
    // Ensure result is non-negative
    let x = (x + n_product) % n_product;

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

    #[test]
    fn test_chinese_remainder_solver_simple() {
        let a: Vec<i64> = vec![2, 3];
        let n: Vec<i64> = vec![3, 5];

        let result = chinese_remainder_solver(a, n).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_chinese_remainder_solver_invalid() {
        // Test length mismatch
        let a: Vec<i64> = vec![2, 3];
        let n: Vec<i64> = vec![3, 5, 7];

        let result = chinese_remainder_solver(a, n);
        assert!(result.is_err());

        // Test non-coprime moduli
        let a: Vec<i64> = vec![2, 3];
        let n: Vec<i64> = vec![3, 6];

        let result = chinese_remainder_solver(a, n);
        assert!(result.is_err());
    }
}

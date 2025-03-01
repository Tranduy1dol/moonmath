use crate::extended_euclid::extended_euclid;
fn chinese_remainder_solver(a: Vec<u64>, n: Vec<u64>) -> anyhow::Result<u64> {
    if n.len() != a.len() {
        anyhow::bail!("Length mismatch");
    }

    let n_product = n.iter().product::<u64>();
    let mut x_prime = 0u64;

    for j in 0..a.len() {
        let a_j = a.get(j).unwrap();
        let n_j = n.get(j).unwrap();

        let n_product_j = n_product / n_j;
        let (_, s_j, _) = extended_euclid(n_product_j, *n_j)?;

        x_prime += a_j * s_j as u64 * n_product_j;
    }

    let x = x_prime % n_product;

    Ok(x)
}

mod test {
    use crate::chinese_remainder::chinese_remainder_solver;

    #[test]
    fn test_chinese_remainder_solver() {
        let a: Vec<u64> = vec![4, 1, 3, 0];
        let n: Vec<u64> = vec![7, 3, 5, 11];

        let result = chinese_remainder_solver(a, n).unwrap();
        assert_eq!(result, 88);
    }
}

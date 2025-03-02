pub fn long_division(dividend: i64, divisor: i64) -> anyhow::Result<(i64, u64)> {
    if divisor == 0 {
        anyhow::bail!("division by zero");
    }

    let mut carry: i64 = 0;
    let mut quotient: i64 = 0;

    for digit in dividend
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
    {
        carry = carry * 10 + digit;
        quotient *= 10;

        while carry >= divisor {
            carry -= divisor;
            quotient += 1;
        }
    }

    Ok((quotient, carry as u64))
}

pub fn poly_long_division(a: Vec<i64>, b: Vec<i64>) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    let mut p = a.clone();
    let mut q = vec![0i64; a.len() - b.len() + 1];

    let d = b.len() as u64 - 1;

    while p.len() as u64 - 1 >= d {
        let s = p.last().unwrap() / b.last().unwrap();
        q[p.len() - b.len()] = s;

        let mut s_product_b = vec![0i64; p.len()];
        let degree_s_product_b = s_product_b.len();
        for i in 0..b.len() {
            s_product_b[degree_s_product_b - b.len() + i] = s * b[i];
        }

        for i in 0..p.len() {
            p[i] -= s_product_b[i];
        }

        if *p.last().unwrap() == 0 {
            p.pop();
        }
    }

    Ok((q, p))
}

mod test {
    use crate::long_division::{long_division, poly_long_division};

    #[test]
    fn test_long_division() {
        let result = long_division(45678, 90).unwrap();
        assert_eq!(result, (507, 48));
    }

    #[test]
    fn test_poly_long_division() {
        let a = vec![-9, 0, 0, 2, 0, 1];
        let b = vec![-1, 4, 1];
        let result = poly_long_division(a, b).unwrap();
        assert_eq!(result, (vec![-80, 19, -4, 1], vec![-89, 339]));
    }
}

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

// pub fn poly_long_division(a: Vec<u64>, b: Vec<u64>) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
//     let lc_b = b.last().unwrap();
//     let d = b.len() as u64;
//     let q: Vec<u64> = vec![];
//     let p = a.clone();
//
//     while p.len() as u64 > d {
//
//     }
//
//     Ok()
// }

mod test {
    use crate::long_division::long_division;

    #[test]
    fn test_long_division() {
        let result = long_division(45678, 90).unwrap();
        assert_eq!(result, (507, 48));
    }
}

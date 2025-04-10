/// Function to perform long division on integers
///
/// # Arguments
///
/// * `dividend` - The number to be divided.
/// * `divisor` - The number by which the dividend is divided.
///
/// # Returns
///
/// A tuple containing the quotient and the remainder.
pub fn long_division(dividend: i64, divisor: i64) -> anyhow::Result<(i64, u64)> {
    if divisor == 0 {
        // Return an error if divisor is zero
        anyhow::bail!("division by zero");
    }

    let mut carry: i64 = 0;
    let mut quotient: i64 = 0;

    // Convert the dividend to a string, then to individual digits
    for digit in dividend
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
    {
        carry = carry * 10 + digit; // Update carry with the current digit
        quotient *= 10; // Shift quotient left by one digit

        // Subtract the divisor from carry until carry is less than divisor
        while carry >= divisor {
            carry -= divisor;
            quotient += 1; // Increment quotient for each subtraction
        }
    }

    // Return the quotient and the remainder (carry)
    Ok((quotient, carry as u64))
}

/// Function to perform polynomial long division
///
/// # Arguments
///
/// * `a` - The dividend polynomial coefficients.
/// * `b` - The divisor polynomial coefficients.
///
/// # Returns
///
/// A tuple containing the quotient and the remainder polynomials.
pub fn poly_long_division(a: Vec<i64>, b: Vec<i64>) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    let mut p = a.clone(); // Clone the dividend polynomial
    let mut q = vec![0i64; a.len() - b.len() + 1]; // Initialize the quotient polynomial

    let d = b.len() as u64 - 1; // Degree of the divisor polynomial

    // Perform the division until the degree of p is less than the degree of b
    while p.len() as u64 > d {
        let s = p.last().unwrap() / b.last().unwrap(); // Leading coefficient of the quotient term
        q[p.len() - b.len()] = s; // Place the quotient term in the correct position

        // Multiply the divisor polynomial by the quotient term
        let mut s_product_b = vec![0i64; p.len()];
        let degree_s_product_b = s_product_b.len();
        for i in 0..b.len() {
            s_product_b[degree_s_product_b - b.len() + i] = s * b[i];
        }

        // Subtract the result from the dividend polynomial
        for i in 0..p.len() {
            p[i] -= s_product_b[i];
        }

        // Remove the leading zero if present
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

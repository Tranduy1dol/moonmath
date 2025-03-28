use crate::utils::elliptic_curve_operations::Point;
use crate::utils::modular_operations::Modular;

/// Performs Lagrange interpolation on a set of points over a given field.
///
/// # Arguments
///
/// * `points` - A vector of `Point` structs representing the points to interpolate.
/// * `field` - A `Modular` struct representing the finite field for operations.
///
/// # Returns
///
/// Performs Lagrange interpolation over a set of points within a finite field.
///
/// Given a collection of points (each with an x and y coordinate), this function computes the coefficients
/// of the polynomial that passes through all the points using Lagrange's formula. All arithmetic operations
/// are carried out in the finite field specified by the provided `Modular` instance.
///
/// # Errors
///
/// Returns an error if a division operation in the finite field arithmetic fails.
///
/// # Examples
///
/// ```
/// // Initialize a finite field with modulus 13.
/// let field = Modular::new(13);
///
/// // Define points for interpolation.
/// let points = vec![
///     Point { x: 0, y: 1 },
///     Point { x: 1, y: 3 },
///     Point { x: 2, y: 5 },
/// ];
///
/// // Compute the polynomial coefficients (constant term first).
/// let poly_coeffs = lagrange_interpolation(points, field).unwrap();
///
/// // Validate the results (replace expected values with the actual coefficients).
/// assert_eq!(poly_coeffs, vec![expected_constant, expected_linear, expected_quadratic]);
/// ```pub fn lagrange_interpolation(points: Vec<Point>, field: Modular) -> anyhow::Result<Vec<i64>> {
    let mut result = vec![0; points.len()];

    for i in 0..points.len() {
        let mut prod = 1;
        let mut term = vec![0; points.len()];

        for j in 0..points.len() {
            if i != j {
                prod = field.mul(prod, field.sub(points[i].x, points[j].x));
            }
        }

        let coeff = field.div(points[i].y, prod)?;
        term[0] = coeff;

        for j in 0..points.len() {
            if i == j {
                continue;
            }

            for k in (1..points.len()).rev() {
                term[k] += term[k - 1];
                term[k - 1] = field.mul(term[k - 1], field.neg(points[j].x));

            }
        }

        for j in 0..points.len() {
            result[j] = field.add(result[j], term[j]);
        }

    }

    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lagrange_interpolation() {
        let points = vec![
            Point::new(1, 1, 1),
            Point::new(2, 4, 1),
            Point::new(3, 9, 1),
        ];

        let field = Modular(13);
        let result = lagrange_interpolation(points, field).unwrap();

        let expected = vec![0, 0, 1];

        assert_eq!(result, expected);
    }
}

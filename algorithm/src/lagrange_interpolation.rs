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
/// A vector of `i64` coefficients representing the interpolated polynomial.
pub fn lagrange_interpolation(points: Vec<Point>, field: Modular) -> anyhow::Result<Vec<i64>> {
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

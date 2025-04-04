use std::cmp::PartialEq;

use crate::modular_operations::Modular;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub enum CurveForm {
    ShortWeierstrass,
    Montgomery,
    TwistedEdwards,
}

pub struct EllipticCurve {
    form: CurveForm,
    field: Modular,
    a: i64,
    b: i64,
}

impl Point {
    /// Creates a new `ProjectivePoint`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the point.
    /// * `y` - The y-coordinate of the point.
    /// * `z` - The z-coordinate of the point.
    ///
    /// # Returns
    ///
    /// A new `ProjectivePoint` with the given coordinates.
    pub fn new(x: i64, y: i64, z: i64) -> Point {
        if z == 1 {
            Point { x, y, z }
        } else {
            Point { x: 0, y: 1, z: 0 }
        }
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl EllipticCurve {
    /// Creates a new `EllipticCurve`.
    ///
    /// # Arguments
    ///
    /// * `form` - The form of the elliptic curve.
    /// * `field` - The finite field for operations.
    /// * `a` - The curve parameter `a`.
    /// * `b` - The curve parameter `b`.
    ///
    /// # Returns
    ///
    /// A new `EllipticCurve` with the given parameters.
    pub fn new(form: CurveForm, field: Modular, a: i64, b: i64) -> Self {
        EllipticCurve { form, field, a, b }
    }

    /// Checks if a point is on the elliptic curve.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check.
    ///
    /// # Returns
    ///
    /// `true` if the point is on the curve, `false` otherwise.
    pub fn is_point_on_curve(&self, point: Point) -> bool {
        if point.z == 0 {
            return true;
        }

        match self.form {
            CurveForm::ShortWeierstrass => {
                (point.y * point.y) % self.field.0
                    == (point.x * point.x * point.x + self.a * point.x + self.b) % self.field.0
            }
            CurveForm::Montgomery => {
                (self.b * point.y * point.y) % self.field.0
                    == (point.x * point.x * point.x + self.a * point.x * point.x + point.x)
                        % self.field.0
            }
            CurveForm::TwistedEdwards => {
                (self.a * point.x * point.x) % self.field.0
                    == (1 + self.b * point.y * point.y * point.x * point.x) % self.field.0
            }
        }
    }

    /// Computes the inverse of a point on the elliptic curve.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to invert.
    ///
    /// # Returns
    ///
    /// An `Option` containing the inverse point if it exists.
    pub fn inverse(&self, point: Point) -> anyhow::Result<Point> {
        if self.is_point_on_curve(point) {
            Ok(Point {
                x: point.x,
                y: self.field.neg(point.y),
                z: point.z,
            })
        } else {
            anyhow::bail!("Cannot find inverse of this point")
        }
    }

    /// Adds two points on the elliptic curve in projective coordinates.
    ///
    /// # Arguments
    ///
    /// * `point_a` - The first projective point.
    /// * `point_b` - The second projective point.
    ///
    /// # Returns
    ///
    /// A `Result` containing the resulting projective point.
    pub fn projective_add(&self, point_1: Point, point_2: Point) -> anyhow::Result<Point> {
        let field = self.field;

        if point_1 == Point::new(0, 1, 0) {
            Ok(point_2)
        } else if point_2 == Point::new(0, 1, 0) {
            Ok(point_1)
        } else {
            let u_1 = point_1.z * point_2.y;
            let u_2 = point_1.y * point_2.z;
            let v_1 = point_1.z * point_2.x;
            let v_2 = point_1.x * point_2.z;

            if v_1 == v_2 {
                if u_1 != u_2 {
                    Ok(Point::new(0, 1, 0))
                } else if point_1.y == 0 {
                    Ok(Point::new(0, 1, 0))
                } else {
                    let w = self.a * point_1.z * point_1.z + 3 * point_1.x * point_1.x;
                    let s = point_1.y * point_1.z;
                    let b = point_1.x * point_1.y * s;
                    let h = w * w - 8 * b;

                    let x = 2 * h * s;
                    let y = w * (4 * b - h) - 8 * point_1.y * point_1.y * s * s;
                    let z = 8 * s * s * s;

                    Ok(Point::new(
                        field.div(x, z)?,
                        field.div(y, z)?,
                        field.div(z, z)?,
                    ))
                }
            } else {
                let u = u_1 - u_2;
                let v = v_1 - v_2;
                let w = point_1.z * point_2.z;
                let a = u * u * w - v * v * v - 2 * v * v * v_2;

                let x = v * a;
                let y = u * (v * v * v_2 - a) - v * v * v * u_2;
                let z = v * v * v * w;

                Ok(Point::new(
                    self.field.div(x, z)?,
                    self.field.div(y, z)?,
                    self.field.div(z, z)?,
                ))
            }
        }
    }

    /// Performs scalar multiplication on the elliptic curve.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to be multiplied.
    /// * `scalar` - The scalar value to multiply the point by.
    ///
    /// # Returns
    ///
    /// A `Result` containing the resulting point after scalar multiplication.
    ///
    /// # Errors
    ///
    /// Returns an error if the point is not on the curve.
    pub fn esm(&self, point: Point, mut scalar: i64) -> anyhow::Result<Point> {
        if !self.is_point_on_curve(point) {
            anyhow::bail!("Point is not on the curve");
        }

        if scalar == 0 {
            return Ok(Point::new(0, 1, 0));
        }

        let mut result = Point::new(0, 1, 0);
        let mut base = point;

        while scalar > 0 {
            if scalar & 1 == 1 {
                result = self.projective_add(result, base)?;
            }
            base = self.projective_add(base, base)?;
            scalar >>= 1;
        }

        Ok(result)
    }
}

mod test {
    use super::*;
    use crate::elliptic_curve_operations::CurveForm::ShortWeierstrass;
    use crate::modular_operations::Modular;

    #[test]
    fn test_scalar_multiplication() {
        let e = EllipticCurve::new(ShortWeierstrass, Modular(13), 8, 8);
        assert_eq!(
            e.esm(Point::new(5, 11, 1), 10).unwrap(),
            Point::new(0, 1, 0)
        );
        assert_eq!(e.esm(Point::new(9, 4, 1), 10).unwrap(), Point::new(4, 0, 1));
        assert_eq!(e.esm(Point::new(9, 4, 1), 4).unwrap(), Point::new(7, 11, 1));
    }

    #[test]
    fn test_projective_add() {
        let e = EllipticCurve::new(ShortWeierstrass, Modular(5), 1, 1);
        assert_eq!(
            e.projective_add(Point::new(0, 1, 0), Point::new(4, 3, 1))
                .unwrap(),
            Point::new(4, 3, 1)
        );
        assert_eq!(
            e.projective_add(Point::new(0, 3, 0), Point::new(3, 1, 2))
                .unwrap(),
            Point::new(3, 1, 2)
        );
        assert_eq!(
            e.projective_add(e.inverse(Point::new(0, 4, 1)).unwrap(), Point::new(3, 4, 1))
                .unwrap(),
            Point::new(3, 1, 1)
        );
        assert_eq!(
            e.projective_add(Point::new(4, 3, 1), Point::new(4, 2, 1))
                .unwrap(),
            Point::new(0, 1, 0)
        );
    }

    #[test]
    fn test_inverse_point() {
        let e = EllipticCurve::new(ShortWeierstrass, Modular(5), 1, 1);
        assert_eq!(e.inverse(Point::new(0, 4, 1)).unwrap(), Point::new(0, 1, 1));
    }
}

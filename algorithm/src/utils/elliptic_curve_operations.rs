use crate::utils::modular_operations::Modular;
use std::cmp::PartialEq;

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
    /// Creates a new `Point` in projective coordinates.
    ///
    /// When the provided `z` value equals 1, the point is initialized using the given `x`, `y`, and `z` values.
    /// For any other `z` value, the function returns the identity point (0, 1, 0), representing the point at infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// let point = Point::new(3, 5, 1);
    /// assert_eq!(point.x, 3);
    /// assert_eq!(point.y, 5);
    /// assert_eq!(point.z, 1);
    ///
    /// let identity = Point::new(3, 5, 0);
    /// assert_eq!(identity.x, 0);
    /// assert_eq!(identity.y, 1);
    /// assert_eq!(identity.z, 0);
    /// ```    pub fn new(x: i64, y: i64, z: i64) -> Point {
        if z == 1 {
            Point { x, y, z }
        } else {
            Point {
                x: 0,
                y: 1,
                z: 0,
            }
        }
    }
}

impl PartialEq<Point> for Point {
    /// Compares two `Point` instances for equality by checking if their `x`, `y`, and `z` coordinates are identical.
    ///
    /// # Examples
    ///
    /// ```
    /// let p1 = Point::new(3, 4, 1);
    /// let p2 = Point::new(3, 4, 1);
    /// let p3 = Point::new(5, 6, 1);
    /// assert!(p1.eq(&p2));
    /// assert!(!p1.eq(&p3));
    /// ```
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
    /// Constructs a new `EllipticCurve` instance with the specified curve form, finite field, and coefficients.
    ///
    /// # Arguments
    ///
    /// * `form` - The type of elliptic curve (e.g., `ShortWeierstrass`, `Montgomery`, or `TwistedEdwards`).
    /// * `field` - The finite field over which the curve is defined.
    /// * `a` - The curve coefficient `a`.
    /// * `b` - The curve coefficient `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{EllipticCurve, CurveForm, Modular};
    ///
    /// // Create a new elliptic curve with specified parameters.
    /// let field = Modular::new(97);
    /// let curve = EllipticCurve::new(CurveForm::ShortWeierstrass, field, -3, 5);
    /// ```    pub fn new(form: CurveForm, field: Modular, a: i64, b: i64) -> Self {
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
    /// Determines whether the provided point satisfies the elliptic curve equation for the curve's specific form.
    /// 
    /// A point is considered to be on the curve if either it represents the point at infinity (its `z` coordinate is 0)
    /// or it fulfills the corresponding curve equation based on the curve form (Short Weierstrass, Montgomery, or Twisted Edwards).
    /// 
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::{Point, EllipticCurve, CurveForm, Modular};
    /// 
    /// // Example using a Short Weierstrass curve over a field with modulus 17.
    /// let field = Modular(17);
    /// let curve = EllipticCurve::new(CurveForm::ShortWeierstrass, field, 2, 3);
    /// 
    /// // A valid point on the curve: for x = 2, y = 7, the curve equation holds:
    /// // 7² mod 17 == (2³ + 2*2 + 3) mod 17  => 49 mod 17 == 15 mod 17, since 49 mod 17 equals 15.
    /// let point = Point::new(2, 7, 1);
    /// assert!(curve.is_point_on_curve(point));
    /// 
    /// // The point at infinity (represented by z == 0) is always considered to be on the curve.
    — let infinity = Point::new(0, 0, 0);
    /// assert!(curve.is_point_on_curve(infinity));
    /// ```    pub fn is_point_on_curve(&self, point: Point) -> bool {
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
                    == (point.x * point.x * point.x + self.a * point.x + point.x % self.field.0)
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
    /// Computes the inverse of a point on the elliptic curve by negating its y-coordinate.
    ///
    /// The inverse of a point (x, y, z) is defined as (x, -y, z), where the negation is performed
    /// using the curve's underlying modular field arithmetic. This method returns an error if the
    /// provided point does not lie on the curve.
    ///
    /// # Examples
    ///
    /// ```
    /// // Replace `Modular`, `CurveForm`, and curve parameters with actual implementations as needed.
    /// let field = Modular::new(17); // assuming a modulus of 17 for demonstration
    /// let curve = EllipticCurve::new(CurveForm::ShortWeierstrass, field, 2, 3);
    /// let point = Point::new(5, 1, 1);
    /// let inverse_point = curve.inverse(point).unwrap();
    /// assert_eq!(inverse_point, Point::new(5, curve.field.neg(1), 1));
    /// ```    pub fn inverse(&self, point: Point) -> anyhow::Result<Point> {
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
    /// Computes the sum of two projective points on the elliptic curve.
    ///
    /// This method performs projective addition, handling both the generic case of adding
    /// two distinct points and the special case of point doubling. The identity element is
    /// represented by `Point(0, 1, 0)`; if either input equals this identity, the other point is returned.
    /// 
    /// Finite field arithmetic is employed for the computations, and modular division is used to
    /// derive the resulting coordinates. Any error encountered during field division is propagated
    /// via the returned `Result`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example demonstrating projective point addition.
    /// let field = Modular::new(97); // Example modulus.
    /// let curve = EllipticCurve::new(CurveForm::ShortWeierstrass, field, 2, 3);
    /// let point1 = Point::new(3, 6, 1);
    /// let point2 = Point::new(10, 1, 1);
    /// let sum_point = curve.projective_add(point1, point2).unwrap();
    /// // `sum_point` now represents the projective sum of `point1` and `point2`.
    /// ```    pub fn projective_add(
        &self,
        point_1: Point,
        point_2: Point,
    ) -> anyhow::Result<Point> {
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
    /// Performs scalar multiplication on an elliptic curve point using the double-and-add algorithm.
    ///
    /// This method first validates that the input point lies on the curve. If the point is not on
    /// the curve, an error is returned. A scalar of zero results in the identity point (`Point::new(0, 1, 0)`).
    /// When the scalar is positive, the method iteratively adds and doubles the point until the scalar
    /// has been fully processed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use algorithm::elliptic_curve_operations::{EllipticCurve, Point, CurveForm};
    /// # use modular::Modular;
    /// // Define an example elliptic curve over a finite field.
    /// let field = Modular::new(23);
    /// let curve = EllipticCurve::new(CurveForm::ShortWeierstrass, field, 1, 1);
    ///
    /// // Create a point on the curve.
    /// let point = Point::new(3, 10, 1);
    ///
    /// // Compute the scalar multiplication of the point (e.g., 5 * point).
    /// let result = curve.esm(point, 5).unwrap();
    ///
    /// // `result` holds the resulting point after multiplying by the scalar.
    /// ```    pub fn esm(&self, point: Point, mut scalar: i64) -> anyhow::Result<Point> {
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
    use crate::utils::elliptic_curve_operations::{CurveForm, EllipticCurve, Point};
    use crate::utils::elliptic_curve_operations::CurveForm::ShortWeierstrass;
    use crate::utils::modular_operations::Modular;

    #[test]
    fn test_scalar_multiplication() {
        let e = EllipticCurve::new(CurveForm::ShortWeierstrass, Modular(13), 8, 8);
        assert_eq!(e.esm(Point::new(5, 11, 1), 10).unwrap(), Point::new(0, 1, 0));
        assert_eq!(e.esm(Point::new(9, 4, 1), 10).unwrap(), Point::new(4, 0, 1));
        assert_eq!(e.esm(Point::new(9, 4, 1), 4).unwrap(), Point::new(7, 11, 1));
    }

    #[test]
    fn test_projective_add() {
        let e = EllipticCurve::new(ShortWeierstrass, Modular(5), 1, 1);
        assert_eq!(e.projective_add(Point::new(0, 1, 0), Point::new(4, 3, 1)).unwrap(), Point::new(4, 3, 1));
        assert_eq!(e.projective_add(Point::new(0, 3, 0), Point::new(3, 1, 2)).unwrap(), Point::new(3, 1, 2));
        assert_eq!(e.projective_add(e.inverse(Point::new(0, 4, 1)).unwrap() , Point::new(3, 4, 1)).unwrap(), Point::new(3, 1, 1));
        assert_eq!(e.projective_add(Point::new(4, 3, 1), Point::new(4, 2, 1)).unwrap(), Point::new(0, 1, 0));
    }

    #[test]
    fn test_inverse_point() {
        let e = EllipticCurve::new(ShortWeierstrass, Modular(5), 1, 1);
        assert_eq!(e.inverse(Point::new(0, 4, 1)).unwrap(), Point::new(0, 1, 1));
    }
}


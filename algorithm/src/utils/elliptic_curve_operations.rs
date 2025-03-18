use crate::utils::modular_operations::Modular;
use anyhow::anyhow;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ProjectivePoint {
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
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl ProjectivePoint {
    pub fn new(x: i64, y: i64, z: i64) -> ProjectivePoint {
        ProjectivePoint { x, y, z }
    }
}

impl PartialEq<Point> for &Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<ProjectivePoint> for ProjectivePoint {
    fn eq(&self, other: &ProjectivePoint) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl EllipticCurve {
    pub fn new(form: CurveForm, field: Modular, a: i64, b: i64) -> Self {
        EllipticCurve { form, field, a, b }
    }

    pub fn point_at_infinity(&self) -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn is_point_on_curve(&self, point: &Point) -> bool {
        if point == self.point_at_infinity() {
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

    pub fn inverse(&self, point: &Point) -> Option<Point> {
        if self.is_point_on_curve(point) {
            Some(Point {
                x: point.x,
                y: self.field.sub(0, point.y) as i64,
            })
        } else {
            None
        }
    }

    pub fn tangent_add(&self, point: &Point) -> anyhow::Result<Point> {
        if !self.is_point_on_curve(point) {
            eprintln!("point = {:#?}", point);
            return Err(anyhow!("Point is not on the curve"));
        }

        match self.form {
            CurveForm::ShortWeierstrass => {
                let pattern = self
                    .field
                    .div(3 * point.x * point.x + self.a, 2 * point.y)
                    .unwrap() as i64;

                let new_x = self.field.sub(pattern * pattern, 2 * point.x) as i64;

                let new_y = self.field.sub(pattern * (point.x - new_x), point.y) as i64;

                let result = Point::new(new_x, new_y);
                Ok(result)
            }
            CurveForm::Montgomery => {
                let pattern = self
                    .field
                    .div(3 * point.x * point.x + 2 * self.a + 1, 2 * self.a * point.y)
                    .unwrap() as i64;

                let new_x = self
                    .field
                    .sub(pattern * pattern * self.b, 2 * point.x - self.a)
                    as i64;

                let new_y = self.field.sub(pattern * (point.x - new_x), point.y) as i64;

                let result = Point::new(new_x, new_y);
                Ok(result)
            }
            CurveForm::TwistedEdwards => {
                let result = self.chord_add(point, point)?;
                Ok(result)
            }
        }
    }

    pub fn chord_add(&self, point_a: &Point, point_b: &Point) -> anyhow::Result<Point> {
        if !self.is_point_on_curve(point_a) || !self.is_point_on_curve(point_b) {
            return Err(anyhow!("Point is not on the curve"));
        }

        if point_a == self.point_at_infinity() {
            return Ok(*point_b);
        }

        if point_b == self.point_at_infinity() {
            return Ok(*point_a);
        }

        match self.form {
            CurveForm::ShortWeierstrass => {
                if point_a.x == point_b.x {
                    if point_a.y == point_b.y {
                        Err(anyhow!("x_1 and x_2 should not be equal"))
                    } else {
                        Ok(self.tangent_add(point_a)?)
                    }
                } else {
                    let pattern = self
                        .field
                        .div(point_b.y - point_a.y, point_b.x - point_a.x)
                        .unwrap() as i64;

                    let new_x = self.field.sub(pattern * pattern, point_a.x + point_b.x) as i64;

                    let new_y = self.field.sub(pattern * (point_a.x - point_b.x), point_a.y) as i64;

                    Ok(Point::new(new_x, new_y))
                }
            }
            CurveForm::Montgomery => {
                if point_a.x == point_b.x {
                    if point_a.y == point_b.y {
                        Err(anyhow!("x_1 and x_2 should not be equal"))
                    } else {
                        Ok(self.tangent_add(point_a)?)
                    }
                } else {
                    let pattern = self
                        .field
                        .div(point_b.y - point_a.y, point_b.x - point_a.x)
                        .unwrap() as i64;

                    let new_x = self
                        .field
                        .sub(pattern * pattern * self.b, point_a.x + point_b.x + self.a)
                        as i64;

                    let new_y = self.field.sub(pattern * (point_a.x - new_x), point_b.y) as i64;

                    Ok(Point::new(new_x, new_y))
                }
            }
            CurveForm::TwistedEdwards => {
                let new_x = self
                    .field
                    .div(
                        point_a.x * point_b.y + point_b.x * point_a.y,
                        1 + self.b * point_a.x * point_b.y * point_b.x * point_a.y,
                    )
                    .unwrap() as i64;

                let new_y = self
                    .field
                    .div(
                        point_b.y * point_a.y - self.a * point_a.x * point_b.x,
                        1 - self.b * point_a.y * point_a.x * point_b.y,
                    )
                    .unwrap() as i64;

                Ok(Point::new(new_x, new_y))
            }
        }
    }

    pub fn projective_add(&self, point_a: ProjectivePoint, point_b: ProjectivePoint) -> anyhow::Result<ProjectivePoint> {
        if point_a == ProjectivePoint::new(0, 1, 0) { Ok(point_b) }
        else if point_b == ProjectivePoint::new(0, 1, 0) { Ok(point_a) } else {
            let u_1 = point_a.z * point_b.y;
            let u_2 = point_a.y * point_b.z;
            let v_1 = point_a.z * point_b.x;
            let v_2 = point_a.x * point_b.z;

            if v_1 == v_2 {
                if u_1 != u_2 {
                    Ok(ProjectivePoint::new(0, 1, 0))
                } else {
                    if point_a.y == 0 {
                        Ok(ProjectivePoint::new(0, 1, 0))
                    } else {
                        let w = self.a * point_a.z * point_a.z + 3 * point_a.x * point_a.x;
                        let s = point_a.y * point_b.z;
                        let b = point_a.x * point_a.y * s;
                        let h = w * w - 8 * b;

                        let x = self.field.mul(8, h * s) as i64;
                        let y = self.field.sub(w * (4 * b - h), 8 * point_a.y * point_a.y * s * s) as i64;
                        let z = self.field.mul(8, s * s * s) as i64;

                        Ok(ProjectivePoint::new(x, y, z))
                    }
                }
            } else {
                let u = u_1 - u_2;
                let v = v_1 - v_2;
                let w = point_a.z * point_b.z;
                let a = u * u * w - v * v * v -2 * v * v * v_2;

                let x = self.field.mul(v, a) as i64;
                let y = self.field.sub(u * (v * v - a), v * v * v * u_2) as i64;
                let z = self.field.mul(v * v * v, w) as i64;
                Ok(ProjectivePoint::new(x, y, z))
            }
        }
    }

    pub fn esm(&self, point: &Point, mut scalar: i64) -> anyhow::Result<Point> {
        if !self.is_point_on_curve(point) {
            return Err(anyhow!("Point is not on the curve"));
        }

        if scalar == 0 {
            return Ok(self.point_at_infinity());
        }

        let mut result = self.point_at_infinity();
        let mut base = *point;

        while scalar > 0 {
            if scalar & 1 == 1 {
                result = self.chord_add(&result, &base)?;
            }
            base = self.tangent_add(&base)?;
            scalar >>= 1;
        }

        Ok(result)
    }
}

mod test {
    use crate::utils::elliptic_curve_operations::{CurveForm, EllipticCurve, Point};
    use crate::utils::modular_operations::Modular;

    #[test]
    fn test_curve_chord_add_operations() {
        let e = EllipticCurve::new(CurveForm::ShortWeierstrass, Modular(5), 1, 1);
        let expected_result = Point::new(2, 1);
        let result = e.chord_add(&Point::new(0, 1), &Point::new(4, 2)).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_curve_tangent_operations() {
        let e = EllipticCurve::new(CurveForm::ShortWeierstrass, Modular(5), 1, 1);
        let expected_result = Point::new(3, 4);
        let result = e.tangent_add(&Point::new(4, 2)).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_scalar_multiplication() {
        let e = EllipticCurve::new(CurveForm::ShortWeierstrass, Modular(13), 8, 8);
        let point_1 = Point::new(5, 11);
        assert_eq!(e.esm(&point_1, 10).unwrap(), Point::new(0, 0));
        let point_2 = Point::new(9, 4);
        assert_eq!(e.esm(&point_2, 10).unwrap(), Point::new(4, 0));
        let point_3 = Point::new(9, 4);
        assert_eq!(e.esm(&point_3, 4).unwrap(), Point::new(7, 11));
    }
}

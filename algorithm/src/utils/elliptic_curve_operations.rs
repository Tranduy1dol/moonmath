use crate::utils::modular_operations::Modular;
use anyhow::anyhow;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    pub x: i64,
    pub y: i64,
}

enum CurveForm {
    ShortWeierstrass,
    Montgomery,
    TwistedEdwards,
}

struct EllipticCurve {
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

impl PartialEq<Point> for &Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
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

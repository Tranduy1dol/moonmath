mod test {
    use utils::elliptic_curve_operations::CurveForm::ShortWeierstrass;
    use utils::elliptic_curve_operations::{EllipticCurve, Point};
    use utils::modular_operations::Modular;

    #[test]
    pub fn exercise_65() {
        let tini_jubjub = EllipticCurve::new(
            ShortWeierstrass,
            Modular(13),
            8,
            8
        );

        let point1 = tini_jubjub.esm(Point::new(5, 11, 1), 10).unwrap();
        let point2 = tini_jubjub.esm(Point::new(9, 4, 1), 10).unwrap();
        let point3 = tini_jubjub.esm(Point::new(9, 4, 1), 4).unwrap();

        println!("[10](5, 11) = ({:}, {:})", point1.x, point1.y);
        println!("[10](9, 4) = ({:}, {:})", point2.x, point2.y);
        println!("[4](9, 4) = ({:}, {:})", point3.x, point3.y);
    }

    #[test]
    pub fn exercise_67() {
        let e = EllipticCurve::new(ShortWeierstrass, Modular(13), 8, 8);
        let mut logarithm_order: Vec<Point> = Vec::new();

        let mut point = Point::new(0, 1, 0);
        while e.is_point_on_curve(point) {
            let generator = Point::new(7, 11, 1);
            point = e.projective_add(generator, point).unwrap();
            logarithm_order.push(point);
            if point.is_infinity() {
                break;
            }
        }

        println!("Logarithm order: {:?}", logarithm_order);
    }

    #[test]
    pub fn exercise_68() {
        let e = EllipticCurve::new(ShortWeierstrass, Modular(5), 1, 1);
        println!("[0, 1, 0] + [4, 3, 1] = {:?}", e.projective_add(Point::new(0, 1, 0), Point::new(4, 3, 1)));
        println!("[0, 3, 0] + [3, 1, 2] = {:?}", e.projective_add(Point::new(0, 3, 0), Point::new(3, 1, 2)));
        println!("-[0, 4, 1] + [3, 4, 1] = {:?}", e.projective_add(e.inverse(Point::new(0, 4, 1)).unwrap(), Point::new(3, 4, 1)));
        println!("[4, 3, 1] + [4, 2, 1] = {:?}", e.projective_add(Point::new(4, 3, 1), Point::new(4, 2, 1)));
    }
}
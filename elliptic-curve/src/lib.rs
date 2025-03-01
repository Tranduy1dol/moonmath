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
}
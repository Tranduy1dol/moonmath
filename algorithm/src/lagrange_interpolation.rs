use crate::utils::elliptic_curve_operations::Point;

pub fn lagrange_interpolation(points: Vec<Point>) -> Vec<i64> {
    let mut result = vec![0; points.len()];
    for i in 0..points.len() {
        let mut prod = 0;
        let mut term = vec![0; points.len()];
        
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            prod *= points[i].x - points[j].x;
        }

        prod = points[i].y / prod;
        term[0] = prod;

        for j in 0..points.len() {
            if i == j {
                continue;
            }
            for k in points.len() - 1..0 {
                term[k] += term[k - 1];
                term[k - 1] *= -points[j].x;
            }
        }

        for j in 0..points.len() {
            result[j] += term[j];
        }
    }
    result
}
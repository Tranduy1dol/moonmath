use std::mem::swap;

pub fn extended_euclid(mut a: u64, mut b: u64) -> anyhow::Result<(u64, i64, i64)> {
    if a < b {
        swap(&mut a, &mut b)
    }

    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);

    while r1 != 0 {
        let q = (r0 / r1) as i64;
        let (r2, s2, t2) = (r0 % r1, s0 - q * s1, t0 - q * t1);
        r0 = r1;
        r1 = r2;
        s0 = s1;
        s1 = s2;
        t0 = t1;
        t1 = t2;
    }

    Ok((r0, s0, t0))
}

mod test {
    use crate::extended_euclid::extended_euclid;

    #[test]
    fn test_extended_euclid() {
        let result = extended_euclid(12, 5).unwrap();
        assert_eq!(result, (1, -2, 5))
    }

    #[test]
    fn test_extended_euclid2() {
        let result = extended_euclid(45, 10).unwrap();
        assert_eq!(result, (5, 1, -4))
    }
}

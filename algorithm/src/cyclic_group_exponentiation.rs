pub fn cge(g: u64, mut x: u64, n: u64) -> anyhow::Result<u64> {
    let mut h = g;
    x >>= 1;

    while x > 0 {
        h = (h * h) % n;
        if x & 1 == 1 {
            h = (h * g) % n;
        }
        x >>= 1;
    }

    Ok(h)
}

pub fn esm(g: u64, mut x: u64, n: u64) -> anyhow::Result<u64> {
    let mut h = g;
    x >>= 1;

    while x > 0 {
        h = (h + h) % n;
        if x & 1 == 1 {
            h = (h * g) % n;
        }
        x >>= 1;
    }

    Ok(h)
}
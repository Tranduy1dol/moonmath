use sha2::{Digest, Sha256};

pub fn hash_to_zn(n: u128, k: usize, s: &[u8]) -> u128 {
    let mut c = 0u128;
    loop {
        let mut s_prime = Vec::from(s);
        s_prime.extend(c.to_le_bytes());

        let hash = Sha256::digest(&s_prime);

        let mut z = 0u128;
        for (i, byte) in hash.iter().enumerate().take((k + 7) / 8) {
            z |= (*byte as u128) << (i * 8);
        }
        z &= (1 << k) - 1;

        if z < n {
            return z;
        }

        c += 1;
    }
}

mod test {
    use crate::hash_to_zn::hash_to_zn;

    #[test]
    fn test_hash_to_zn() {
        let n = 1000;
        let k = 16;
        let seed = b"example_seed";

        let result = hash_to_zn(n, k, seed);
        println!("z = {}", result);
    }
}
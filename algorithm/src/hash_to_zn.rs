use sha2::{Digest, Sha256};

/// Generates a pseudorandom number derived from a seed that is less than a specified upper bound.
///
/// This function appends an incrementing counter (in little-endian byte order) to the provided seed and computes its
/// SHA-256 hash. It extracts the first `k` bits from the hash and masks the result to ensure it fits within `k` bits.
/// If the computed number is less than `n`, it is returned; otherwise, the process repeats with an incremented counter.
///
/// # Parameters
///
/// - `n`: The exclusive upper bound for the generated number.
/// - `k`: The number of bits to extract from the SHA-256 hash output.
/// - `s`: A byte slice serving as the seed for hash computation.
///
/// # Examples
///
/// ```
/// use algorithm::hash_to_zn;
///
/// let n = 1000u128;
/// let k = 16;
/// let seed = b"example_seed";
/// let result = hash_to_zn(n, k, seed);
/// assert!(result < n);
/// ```
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

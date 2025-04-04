/// Converts a given integer to its binary representation as a string.
///
/// # Arguments
///
/// * `input` - The integer to be converted.
///
/// # Returns
///
/// A string representing the binary form of the input integer.
pub fn binary_representation(mut input: usize) -> anyhow::Result<String> {
    let mut result: String = "".to_string();
    while !input.eq(&0) {
        let char = input % 2;
        result.push(if char == 1 { '1' } else { '0' });
        input /= 2;
    }

    if result.is_empty() {
        return Ok("0".to_string());
    }

    Ok(result.chars().rev().collect())
}

mod test {
    use crate::binary_representation::binary_representation;

    #[test]
    fn test_binary_representation() {
        let result = binary_representation(2).unwrap();
        assert_eq!(result, "10".to_string());
    }

    #[test]
    fn test_binary_representation2() {
        let result = binary_representation(3).unwrap();
        assert_eq!(result, "11".to_string());
    }

    #[test]
    fn test_binary_representation_zero() {
        let result = binary_representation(0).unwrap();
        assert_eq!(result, "0".to_string());
    }

    #[test]
    fn test_binary_representation_large_number() {
        let result = binary_representation(255).unwrap();
        assert_eq!(result, "11111111".to_string());
    }
}

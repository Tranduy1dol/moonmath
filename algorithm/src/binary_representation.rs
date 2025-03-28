/// Converts a given integer to its binary representation as a string.
///
/// # Arguments
///
/// * `input` - The integer to be converted.
///
/// # Returns
///
/// Converts an unsigned integer to its binary representation string with digits in reverse order.
///
/// This function repeatedly divides the input by 2 and appends each remainder (0 or 1) as a character
/// to the result string. As a consequence, the binary digits are ordered from least significant to most significant.
/// For example, an input of 2 produces the string "01".
///
/// # Examples
///
/// ```
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// let binary = binary_representation(2)?;
/// assert_eq!(binary, "01");
/// # Ok(()) }
/// ```pub fn binary_representation(mut input: usize) -> anyhow::Result<String> {
    let mut result: String = "".to_string();
    while !input.eq(&0) {
        let char = input % 2;
        result.push(char.to_string().parse()?);
        input /= 2;
    }
    Ok(result)
}

mod test {
    use crate::binary_representation::binary_representation;

    #[test]
    fn test_binary_representation() {
        let result = binary_representation(2).unwrap();
        assert_eq!(result, "01".to_string());
    }

    #[test]
    fn test_binary_representation2() {
        let result = binary_representation(3).unwrap();
        assert_eq!(result, "11".to_string());
    }
}

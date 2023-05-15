/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.replace(" ", "");

    for val in code.chars() {
        if !val.is_digit(10) {
            return false;
        }
    }

    if code.len() == 1 {
        return false;
    }

    let mut new_code = String::from(&code);
    for (idx, digit) in code.chars().rev().enumerate() {
        let digit: u8 = digit as u8 - '0' as u8;

        if idx % 2 != 0 {
            let mut result: u8 = digit * 2;
            if result > 9 {
                result -= 9;
            }

            let new_code_idx: usize = new_code.len() - idx - 1;
            new_code.replace_range(new_code_idx..=new_code_idx, &result.to_string());
        }
    }

    let mut sum: u8 = 0;
    for digit in new_code.chars() {
        sum += digit as u8 - '0' as u8;
    }

    return sum % 10 == 0;
}

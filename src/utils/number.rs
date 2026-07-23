/// Pad a number to at least length.
fn pad_number(x: usize, pad_length: usize) -> String {
    format!("{:0>len$}", x.to_string(), len = pad_length)
}

/// Pad the year number to at least 4 digits.
pub fn pad_year_number(x: usize) -> String {
    pad_number(x, 4)
}

/// Pad the day number to at least 2 digits.
pub fn pad_day_number(x: usize) -> String {
    pad_number(x, 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_year_padding() {
        assert_eq!(pad_year_number(2), "0002");
        assert_eq!(pad_year_number(69), "0069");
        assert_eq!(pad_year_number(727), "0727");
        assert_eq!(pad_year_number(2077), "2077");
        assert_eq!(pad_year_number(69420), "69420");
    }

    #[test]
    fn test_day_padding() {
        assert_eq!(pad_day_number(2), "02");
        assert_eq!(pad_day_number(69), "69");
        assert_eq!(pad_day_number(727), "727");
    }
}

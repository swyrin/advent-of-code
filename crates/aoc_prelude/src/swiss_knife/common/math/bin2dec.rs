/// Convert a [`&[usize]`] to decimal.
///
/// This one is big-endian, so 0-th array element holds the MSB, make sure to call an
/// extra [`Iterator::rev`] if this is against your intention.
///
/// (Yes, shit design: I want the data to look like how I look at them)
pub fn bin_convert_to_dec_be(bits: &[usize]) -> usize {
    bits.iter().fold(0usize, |current, &bit| (current << 1) | bit)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_to_dec_conversion() {
        assert_eq!(bin_convert_to_dec_be(&[1, 0, 0, 1]), 9);
        assert_eq!(bin_convert_to_dec_be(&[0, 0, 0, 1]), 1);
        assert_eq!(bin_convert_to_dec_be(&[1]), 1);
        assert_eq!(bin_convert_to_dec_be(&[1, 0, 0, 0]), 8);
    }
}

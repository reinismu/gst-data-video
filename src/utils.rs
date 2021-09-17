use once_cell::sync::Lazy;

use self::convert::Convert;

mod convert;

static BASE_256_255: Lazy<Convert> = Lazy::new(|| Convert::new(256, 255));
static BASE_255_256: Lazy<Convert> = Lazy::new(|| Convert::new(255, 256));

pub fn convert_without_zeros(x: u32) -> u32 {
    let bytes = u32::to_le_bytes(x);

    let output_bytes: Vec<u8> = BASE_256_255.convert::<u8, u8>(&bytes);
    u32::from_le_bytes([
        output_bytes[0] + 1,
        output_bytes[1] + 1,
        output_bytes[2] + 1,
        output_bytes[3] + 1,
    ])
}

pub fn convert_back_with_zeros(x: u32) -> u32 {
    let bytes = u32::to_le_bytes(x);
    let bytes = &[bytes[0] - 1, bytes[1] - 1, bytes[2] - 1, bytes[3] - 1];

    let mut output_bytes: Vec<u8> = BASE_255_256.convert::<u8, u8>(bytes);
    output_bytes.push(0);

    u32::from_le_bytes([
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_without_zeros_works() {
        assert_eq!(u32::from_be_bytes([0, 0, 0, 255]), 255);
        assert_eq!(u32::from_le_bytes([255, 0, 0, 0]), 255);
        assert_eq!(
            u32::to_be_bytes(convert_without_zeros(u32::from_be_bytes([0, 0, 0, 0]))),
            [1, 1, 1, 1]
        );
        assert_eq!(
            u32::to_be_bytes(convert_without_zeros(u32::from_be_bytes([0, 0, 0, 1]))),
            [1, 1, 1, 2]
        );
        assert_eq!(
            u32::to_be_bytes(convert_without_zeros(u32::from_be_bytes([0, 0, 0, 255]))),
            [1, 1, 2, 1]
        );
    }

    #[test]
    fn convert_back_with_zeros_works() {
        assert_eq!(convert_back_with_zeros(convert_without_zeros(0)), 0);
        assert_eq!(convert_back_with_zeros(convert_without_zeros(1)), 1);
        assert_eq!(convert_back_with_zeros(convert_without_zeros(255)), 255);
        assert_eq!(
            convert_back_with_zeros(convert_without_zeros(345634)),
            345634
        );
    }
}

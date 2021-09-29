use once_cell::sync::Lazy;

use self::convert::Convert;

mod convert;

pub const MAGIC_NUMBER: u32 = 0xDEADB00B;

static BASE_256_254: Lazy<Convert> = Lazy::new(|| Convert::new(256, 254));
static BASE_254_256: Lazy<Convert> = Lazy::new(|| Convert::new(254, 256));

pub fn convert_without_0_and_255(x: u32) -> u32 {
    let bytes = u32::to_le_bytes(x);

    let output_bytes: Vec<u8> = BASE_256_254.convert::<u8, u8>(&bytes);
    u32::from_le_bytes([
        output_bytes[0] + 1,
        output_bytes[1] + 1,
        output_bytes[2] + 1,
        output_bytes[3] + 1,
    ])
}

pub fn convert_back_with_0_and_255(x: u32) -> u32 {
    let bytes = u32::to_le_bytes(x);
    let bytes = &[bytes[0] - 1, bytes[1] - 1, bytes[2] - 1, bytes[3] - 1];

    let mut output_bytes: Vec<u8> = BASE_254_256.convert::<u8, u8>(bytes);
    output_bytes.push(0);

    u32::from_le_bytes([
        output_bytes[0],
        output_bytes[1],
        output_bytes[2],
        output_bytes[3],
    ])
}

// 0 -> 254 1
// 254 -> 254 2
// 255 -> 254 3
const ESCAPE_BYTE: u8 = 254;

pub fn convert_to_sdi_safe_payload(payload: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(payload.len());
    for b in payload {
        match b {
            0 => {
                output.push(ESCAPE_BYTE);
                output.push(1);
            }
            &ESCAPE_BYTE => {
                output.push(ESCAPE_BYTE);
                output.push(2);
            }
            255 => {
                output.push(ESCAPE_BYTE);
                output.push(3);
            }
            value => {
                output.push(*value);
            }
        }
    }

    output
}

pub fn convert_from_sdi_safe_payload(payload: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(payload.len());
    let mut escape_logic = false;
    for b in payload {
        if escape_logic {
            match b {
                1 => {
                    output.push(0);
                }
                2 => {
                    output.push(254);
                }
                3 => {
                    output.push(255);
                }
                _ => {
                    panic!("Unexpected escape number: {}", b);
                }
            }
            escape_logic = false;
        } else {
            match b {
                254 => {
                    escape_logic = true;
                }
                value => {
                    output.push(*value);
                }
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_and_from_sdi_safe_payload_works() {
        let payload1 = [0, 2, 3, 4, 254, 255];

        assert_eq!(
            convert_to_sdi_safe_payload(&payload1),
            [254, 1, 2, 3, 4, 254, 2, 254, 3]
        );

        assert_eq!(
            convert_from_sdi_safe_payload(&convert_to_sdi_safe_payload(&payload1)),
            [0, 2, 3, 4, 254, 255]
        );

        let mut large_payload: Vec<u8> = vec![1; 1024 * 10];

        large_payload[103] = 255;

        assert_eq!(
            convert_from_sdi_safe_payload(&convert_to_sdi_safe_payload(&large_payload)),
            large_payload
        );
    }

    #[test]
    fn convert_without_zeros_works() {
        assert_eq!(u32::from_be_bytes([0, 0, 0, 255]), 255);
        assert_eq!(u32::from_le_bytes([255, 0, 0, 0]), 255);
        assert_eq!(
            u32::to_be_bytes(convert_without_0_and_255(u32::from_be_bytes([0, 0, 0, 0]))),
            [1, 1, 1, 1]
        );
        assert_eq!(
            u32::to_be_bytes(convert_without_0_and_255(u32::from_be_bytes([0, 0, 0, 1]))),
            [1, 1, 1, 2]
        );
        assert_eq!(
            u32::to_be_bytes(convert_without_0_and_255(u32::from_be_bytes([
                0, 0, 0, 255
            ]))),
            [1, 1, 2, 2]
        );
    }

    #[test]
    fn convert_back_with_zeros_works() {
        assert_eq!(convert_back_with_0_and_255(convert_without_0_and_255(0)), 0);
        assert_eq!(convert_back_with_0_and_255(convert_without_0_and_255(1)), 1);
        assert_eq!(
            convert_back_with_0_and_255(convert_without_0_and_255(255)),
            255
        );
        assert_eq!(
            convert_back_with_0_and_255(convert_without_0_and_255(345634)),
            345634
        );
    }
}

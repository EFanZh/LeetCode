pub mod dynamic_programming;

pub trait Solution {
    fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("10110101", 2, 2), 2),
            (("11111", 2, 3), 0),
            (("110000", 1, 1), 1),
            (("0111101", 1, 2), 3),
            (("1", 1, 1), 0),
            (("0000", 1, 1), 0),
            (
                ("1000000000001000000100111100001101111000000001001111110000000000", 6, 4),
                3,
            ),
            (
                (
                    "11010111011001011110010001011001001101101010100010100010000100101011001001100100111010100001100101001100001000011110101000000101110000000100101001101011001100101100101000101110101100001011000011101001001000001111010111100111000000100011100000110111000000011010011011110000101100001101011000101011110111100110110110000100011000000010011010101001010011001110001000000111010010011000110100100101010010100111000110111100110100100110100101101100010000101001110010000101000100000101110111100110011110101010011111111001110010110011111101001000111",
                    197,
                    346,
                ),
                0,
            ),
        ];

        for ((floor, num_carpets, carpet_len), expected) in test_cases {
            assert_eq!(
                S::minimum_white_tiles(floor.to_string(), num_carpets, carpet_len),
                expected,
            );
        }
    }
}

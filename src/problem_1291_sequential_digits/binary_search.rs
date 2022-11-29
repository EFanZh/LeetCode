pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        static DATA: [i32; 36] = [
            12,
            23,
            34,
            45,
            56,
            67,
            78,
            89,
            123,
            234,
            345,
            456,
            567,
            678,
            789,
            1_234,
            2_345,
            3_456,
            4_567,
            5_678,
            6_789,
            12_345,
            23_456,
            34_567,
            45_678,
            56_789,
            123_456,
            234_567,
            345_678,
            456_789,
            1_234_567,
            2_345_678,
            3_456_789,
            12_345_678,
            23_456_789,
            123_456_789,
        ];

        let start = DATA.partition_point(|&value| (value as u32) < (low as u32));
        let length = DATA[start..].partition_point(|&value| (value as u32) <= (high as u32));

        DATA[start..start + length].to_vec()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        Self::sequential_digits(low, high)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

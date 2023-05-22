pub mod sliding_window;

pub trait Solution {
    fn min_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 3, 2, 4] as &[_], 0),
            (&[1, 5, 0, 10, 14], 1),
            (&[3, 100, 20], 0),
            (&[6, 6, 0, 1, 1, 4, 6], 2),
            (&[20, 66, 68, 57, 45, 18, 42, 34, 37, 58], 31),
            (
                &[
                    511, 526, 975, 111, 71, 913, 238, 734, 256, 351, 454, 312, 24, 502, 165, 322, 154, 567, 756, 63,
                    386, 982, 900, 202, 530, 598, 962, 464, 950, 936, 123, 591, 270, 688, 479, 426,
                ],
                871,
            ),
            (&[53, 60, 100, 85, 16, 68, 64, 31, 37, 78], 47),
            (
                &[
                    598, 558, 255, 872, 978, 176, 854, 753, 704, 245, 896, 959, 438, 810, 433, 595, 354, 539, 195, 619,
                ],
                696,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_difference(nums.to_vec()), expected);
        }
    }
}
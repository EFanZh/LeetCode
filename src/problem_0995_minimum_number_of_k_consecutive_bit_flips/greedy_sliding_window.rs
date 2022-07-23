pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut flipped = false;
        let mut result = 0;

        let (mut window, right) = if (k - 1) * 2 < n {
            let (left, rest) = nums.split_at(k);
            let (middle, right) = rest.split_at(rest.len() - (k - 1));
            let mut window = VecDeque::with_capacity(left.len());

            for &num in left {
                let need_to_flip = (num == 0) ^ flipped;

                if need_to_flip {
                    flipped = !flipped;
                    result += 1;
                }

                window.push_back(need_to_flip);
            }

            for &num in middle {
                flipped ^= window.pop_front().unwrap();

                let need_to_flip = (num == 0) ^ flipped;

                if need_to_flip {
                    flipped = !flipped;
                    result += 1;
                }

                window.push_back(need_to_flip);
            }

            (window, right)
        } else {
            let (left, rest) = nums.split_at(n - (k - 1));
            let (middle, right) = rest.split_at(k - left.len());
            let mut window = VecDeque::with_capacity(left.len());

            for &num in left {
                let need_to_flip = (num == 0) ^ flipped;

                if need_to_flip {
                    flipped = !flipped;
                    result += 1;
                }

                window.push_back(need_to_flip);
            }

            for &num in middle {
                if (num == 0) ^ flipped {
                    return -1;
                }
            }

            (window, right)
        };

        for &num in right {
            flipped ^= window.pop_front().unwrap();

            if (num == 0) ^ flipped {
                return -1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_k_bit_flips(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        while y != 0 {
            let z = x % y;

            x = y;
            y = z;
        }

        x
    }

    pub fn max_score(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let n = nums.len();
        let mut cache = vec![0_u32; 1 << n];

        for bits in 3_u16..1 << n {
            let bit_count = bits.count_ones();

            if bit_count % 2 == 0 {
                let i = bit_count / 2;
                let mut max_score = 0;
                let mut x = bits;

                for steps_1 in 1..bit_count {
                    let bit_1 = x.isolate_lowest_one();
                    let mut y = x & (x - 1);

                    for _ in 0..bit_count - steps_1 {
                        let bit_2 = y.isolate_lowest_one();
                        let prev_score = cache[usize::from(bits ^ bit_1 ^ bit_2)];

                        let score = Self::gcd(
                            nums[bit_1.trailing_zeros() as usize],
                            nums[bit_2.trailing_zeros() as usize],
                        ) * i;

                        max_score = max_score.max(prev_score + score);

                        y &= y - 1;
                    }

                    x &= x - 1;
                }

                cache[usize::from(bits)] = max_score;
            }
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(nums: Vec<i32>) -> i32 {
        Self::max_score(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

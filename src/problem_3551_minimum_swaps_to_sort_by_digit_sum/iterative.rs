pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut nums = nums
            .into_iter()
            .map(|num| {
                let num = num.cast_unsigned();
                let mut sum = 0;
                let mut x = num;

                loop {
                    sum += x % 10;
                    x /= 10;

                    if x == 0 {
                        break;
                    }
                }

                (sum, num)
            })
            .zip(0_u32..)
            .collect::<Box<_>>();

        nums.sort_unstable_by_key(|&(x, _)| x);

        let mut result = 0;

        for start in 0..nums.len() as u32 {
            let mut current = &mut nums[start as usize];

            if current.0.0 != 0 {
                loop {
                    current.0.0 = 0;

                    let next_index = current.1;

                    if next_index == start {
                        break;
                    }

                    result += 1;
                    current = &mut nums[next_index as usize];
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swaps(nums: Vec<i32>) -> i32 {
        Self::min_swaps(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

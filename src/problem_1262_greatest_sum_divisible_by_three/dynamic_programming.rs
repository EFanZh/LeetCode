pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(lhs: u32, rhs: u32) -> u32 {
        if lhs == 0 {
            0
        } else {
            lhs + rhs
        }
    }

    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut cache = (0, 0, 0);

        for num in nums {
            let num = num as u32;

            cache = match num % 3 {
                0 => (cache.0 + num, Self::add(cache.1, num), Self::add(cache.2, num)),
                1 => (
                    cache.0.max(Self::add(cache.2, num)),
                    cache.1.max(cache.0 + num),
                    cache.2.max(Self::add(cache.1, num)),
                ),
                _ => (
                    cache.0.max(Self::add(cache.1, num)),
                    cache.1.max(Self::add(cache.2, num)),
                    cache.2.max(cache.0 + num),
                ),
            };
        }

        cache.0 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        Self::max_sum_div_three(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

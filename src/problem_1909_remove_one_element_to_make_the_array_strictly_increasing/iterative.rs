pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut prev_1 = 0;
        let mut prev_2 = 0;
        let mut iter = nums.iter().copied();

        while let Some(num) = iter.next() {
            if num <= prev_2 {
                let mut prev = if num <= prev_1 { prev_2 } else { num };

                return iter.all(|num| {
                    if num <= prev {
                        false
                    } else {
                        prev = num;

                        true
                    }
                });
            }

            prev_1 = prev_2;
            prev_2 = num;
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_be_increasing(nums: Vec<i32>) -> bool {
        Self::can_be_increasing(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut iter = nums.into_iter();

        if let Some(mut first) = iter.next() {
            while let Some(mut second) = iter.next() {
                if second > first {
                    for third in iter {
                        if third > second {
                            return true;
                        } else if third > first {
                            second = third;
                        } else {
                            first = third;
                        }
                    }

                    break;
                }

                first = second;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn increasing_triplet(nums: Vec<i32>) -> bool {
        Self::increasing_triplet(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

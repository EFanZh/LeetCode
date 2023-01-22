pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut iter = nums.iter().map(|&x| x != 0).enumerate();

        while let Some((mut prev_index, is_one)) = iter.next() {
            if is_one {
                for (i, is_one) in iter {
                    if is_one {
                        if i - prev_index <= k {
                            return false;
                        }

                        prev_index = i;
                    }
                }

                break;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        Self::k_length_apart(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

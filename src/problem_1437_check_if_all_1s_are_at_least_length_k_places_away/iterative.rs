pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        if let Some(mut prev_index) = nums.iter().position(|&x| x != 0) {
            let mut i = prev_index + 1;

            while let Some(&x) = nums.get(i) {
                if x != 0 {
                    if i - prev_index <= k {
                        return false;
                    }

                    prev_index = i;
                }

                i += 1;
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

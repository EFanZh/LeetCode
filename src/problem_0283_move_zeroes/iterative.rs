pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut tail;

        loop {
            if let Some(&num) = nums.get(i) {
                if num == 0 {
                    tail = i;
                    i += 1;

                    break;
                }

                i += 1;
            } else {
                return;
            }
        }

        while let Some(&num) = nums.get(i) {
            if num != 0 {
                nums.swap(tail, i);

                tail += 1;
            }

            i += 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn move_zeroes(nums: &mut Vec<i32>) {
        Self::move_zeroes(nums);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

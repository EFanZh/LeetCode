pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(buffer: &mut [i32], selected: usize, result: &mut Vec<Vec<i32>>) {
        let n = buffer.len();

        if selected < n {
            let mut i = selected;

            loop {
                Self::helper(buffer, selected + 1, result);

                i += 1;

                if i < n {
                    buffer.swap(selected, i);
                } else {
                    break;
                }
            }

            buffer[selected..].rotate_left(1);
        } else {
            result.push(buffer.to_vec());
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut result = Vec::new();

        Self::helper(&mut nums, 0, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

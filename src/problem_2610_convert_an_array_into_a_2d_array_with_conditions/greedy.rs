pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut counts = vec![0_u8; nums.len()].into_boxed_slice();
        let mut result = Vec::<Vec<_>>::new();

        for num in nums {
            let count = &mut counts[num as u32 as usize - 1];
            let old_count = *count;

            *count += 1;

            if let Some(row) = result.get_mut(usize::from(old_count)) {
                row.push(num);
            } else {
                result.push(vec![num]);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::find_matrix(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

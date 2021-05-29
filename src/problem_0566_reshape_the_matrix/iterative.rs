pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        let old_rows = nums.len();
        let old_columns = nums.first().map_or(0, Vec::len);

        if old_rows * old_columns == r * c && old_rows != r {
            let mut iter = nums.into_iter().flatten();

            (0..r).map(|_| iter.by_ref().take(c).collect()).collect()
        } else {
            nums
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        Self::matrix_reshape(nums, r, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

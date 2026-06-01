pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let k = k.cast_unsigned() as usize;
        let mut exists = [false; 201];

        for num in nums {
            exists[num.cast_unsigned() as usize] = true;
        }

        let mut candidate = k;

        while matches!(exists.get(candidate), Some(true)) {
            candidate += k;
        }

        candidate as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        Self::missing_multiple(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

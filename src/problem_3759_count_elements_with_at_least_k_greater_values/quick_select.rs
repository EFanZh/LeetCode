pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned() as usize;
        let n = nums.len();

        (if k == 0 {
            n
        } else {
            let (left, &mut middle, _) = nums.select_nth_unstable(n - k);

            left.iter().rposition(|&x| x != middle).map_or(0, |i| i + 1)
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_elements(nums: Vec<i32>, k: i32) -> i32 {
        Self::count_elements(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

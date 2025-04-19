pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(nums: &[u32], mut k: u32, capacity: u32) -> bool {
        let mut selected = false;

        nums.iter().any(|&num| {
            selected = num <= capacity && !selected;
            k -= u32::from(selected);

            k == 0
        })
    }

    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let k = k as u32;
        let mut left = nums.iter().fold(u32::MAX, |min, &x| min.min(x));
        let mut right = nums.iter().fold(0, |min, &x| min.max(x));

        while left < right {
            let middle = (left + right) / 2;

            if Self::check(&nums, k, middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_capability(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

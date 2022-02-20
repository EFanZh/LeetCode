pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.first()
            .and_then(|first| {
                let pivot = nums.partition_point(|x| x >= first);

                if target < *first {
                    nums[pivot..].binary_search(&target).map(|i| (pivot + i))
                } else {
                    nums[..pivot].binary_search(&target)
                }
                .ok()
            })
            .map_or(-1, |i| i as _)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

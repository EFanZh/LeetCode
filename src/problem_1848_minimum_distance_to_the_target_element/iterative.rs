pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as u32 as usize;
        let (left, right) = nums.split_at(start);

        let left_distance = left
            .iter()
            .rev()
            .position(|&x| x == target)
            .map_or(usize::MAX, |i| i + 1);

        let right_distance = right.iter().position(|&x| x == target).unwrap_or(usize::MAX);

        left_distance.min(right_distance) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        Self::get_min_distance(nums, target, start)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

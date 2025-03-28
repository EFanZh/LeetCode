pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_index = start_index as u32 as usize;
        let (left, right) = words.split_at(start_index);
        let is_target = |x| x == &target;

        let Some(right_distance) = right.iter().chain(left).position(is_target) else {
            return -1;
        };

        let left_distance = right.iter().chain(left).rev().position(is_target).unwrap() + 1;

        left_distance.min(right_distance) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        Self::closest_target(words, target, start_index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

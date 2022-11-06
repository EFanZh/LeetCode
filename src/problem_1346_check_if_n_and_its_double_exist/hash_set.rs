pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen = HashSet::new();

        arr.into_iter().any(|value| {
            if seen.contains(&(value * 2)) || (value % 2 == 0 && seen.contains(&(value / 2))) {
                true
            } else {
                seen.insert(value);

                false
            }
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_if_exist(arr: Vec<i32>) -> bool {
        Self::check_if_exist(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

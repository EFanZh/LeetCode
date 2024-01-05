pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut counts = HashMap::<_, u16>::new();

        for s in &arr {
            counts.entry(s.as_str()).and_modify(|count| *count += 1).or_insert(1);
        }

        arr.iter()
            .filter(|&s| counts[s.as_str()] == 1)
            .nth(k as u32 as usize - 1)
            .map_or_else(String::new, String::clone)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        Self::kth_distinct(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

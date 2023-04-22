pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    #[allow(unused_variables)] // Expected.
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut prev = 0;

        for value in target {
            let value = value as usize;
            let skip = value - prev - 1;

            result.extend(
                iter::repeat("Push")
                    .take(skip)
                    .chain(iter::repeat("Pop").take(skip))
                    .chain(Some("Push"))
                    .map(str::to_string),
            );

            prev = value;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        Self::build_array(target, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

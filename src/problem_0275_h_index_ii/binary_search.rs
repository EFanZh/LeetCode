pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::{mem, ptr};

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();

        citations.first().map_or(0, |first| {
            let end = ptr::from_ref(first) as usize + mem::size_of_val(first) * n;

            let index = citations
                .partition_point(|num| *num < ((end - ptr::from_ref(num) as usize) / mem::size_of_val(num)) as i32);

            (n - index) as _
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn h_index(citations: Vec<i32>) -> i32 {
        Self::h_index(citations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

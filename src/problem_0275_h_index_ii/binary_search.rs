pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::mem;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();

        citations.first().map_or(0, |first| {
            let end = first as *const _ as usize + mem::size_of_val(first) * n;

            let index = citations
                .binary_search_by(|num| {
                    if *num < ((end - num as *const _ as usize) / mem::size_of_val(num)) as i32 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap_err();

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

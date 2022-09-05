pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut state = 0;

        seq.bytes()
            .map(|c| {
                let result = if c == b'(' { state } else { 1 - state };

                state = 1 - state;

                result
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_depth_after_split(seq: String) -> Vec<i32> {
        Self::max_depth_after_split(seq)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

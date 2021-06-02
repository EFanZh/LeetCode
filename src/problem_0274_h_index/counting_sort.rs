pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut counts = vec![0; n];

        for num in citations {
            if num != 0 {
                counts[(num as usize).min(n) - 1] += 1;
            }
        }

        let mut total_count = 0;

        for (i, count) in counts.into_iter().enumerate().rev() {
            total_count += count;

            if total_count > i {
                return (i + 1) as _;
            }
        }

        0
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

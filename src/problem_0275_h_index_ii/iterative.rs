pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();

        for (i, num) in (1..=n as i32).rev().zip(citations) {
            if num >= i {
                return i;
            }
        }

        0
    }
}

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

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(encoded.len() + 1);
        let mut prev = first;
        let mut iter = encoded.into_iter();

        loop {
            result.push(prev);

            if let Some(value) = iter.next() {
                prev ^= value;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        Self::decode(encoded, first)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

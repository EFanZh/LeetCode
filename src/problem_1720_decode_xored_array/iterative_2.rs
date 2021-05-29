pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![0; encoded.len() + 1];
        let (first_target, rest_targets) = result.split_first_mut().unwrap();
        let mut prev = first;

        *first_target = prev;

        for (target, value) in rest_targets.iter_mut().zip(encoded) {
            prev ^= value;
            *target = prev;
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

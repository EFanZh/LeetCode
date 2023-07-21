pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() as i32 + 1;
        let total_xor = i32::from(n & 3 == 1); // Calculates `1 ^ 2 ^ 3 ^ ... ^ n` where `n` is odd.
        let rest_xor = encoded[1..].iter().step_by(2).fold(0, |x, i| x ^ i);
        let mut next = total_xor ^ rest_xor;

        let mut result = encoded;

        for target in &mut result {
            let new_next = next ^ *target;

            *target = next;
            next = new_next;
        }

        result.push(next);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decode(encoded: Vec<i32>) -> Vec<i32> {
        Self::decode(encoded)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

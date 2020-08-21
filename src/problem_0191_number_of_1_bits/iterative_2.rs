pub struct Solution {}

#[allow(non_snake_case)]

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut result = 0;

        while n != 0 {
            result += 1;

            n &= n - 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn hamming_weight(n: u32) -> i32 {
        Self::hammingWeight(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

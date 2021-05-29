pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let num = (num + 1) as _;
        let mut result = vec![0; num];

        for i in 1..num {
            result[i] = result[i & (i - 1)] + 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_bits(num: i32) -> Vec<i32> {
        Self::count_bits(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

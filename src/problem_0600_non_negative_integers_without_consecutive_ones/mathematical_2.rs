pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let fibonacci = [
            1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1_597, 2_584, 4_181, 6_765, 10_946, 17_711,
            28_657, 46_368, 75_025, 121_393, 196_418, 317_811, 514_229, 832_040, 1_346_269, 2_178_309,
        ];

        let fill_bits = 32 - (n & (n >> 1)).leading_zeros();
        let mut result = fibonacci[fill_bits as usize];

        for i in fill_bits..32 - n.leading_zeros() {
            if n & (1 << i) != 0 {
                result += fibonacci[i as usize + 1];
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_integers(n: i32) -> i32 {
        Self::find_integers(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

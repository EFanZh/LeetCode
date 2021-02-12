pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as _
    }
}

impl super::Solution for Solution {
    fn hamming_distance(x: i32, y: i32) -> i32 {
        Self::hamming_distance(x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

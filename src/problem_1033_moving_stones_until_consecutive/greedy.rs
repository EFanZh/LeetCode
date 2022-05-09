pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let (a, b) = if b < a { (b, a) } else { (a, b) };

        let (a, b, c) = if c < a {
            (c, a, b)
        } else if c < b {
            (a, c, b)
        } else {
            (a, b, c)
        };

        let distance_1 = b - a;
        let distance_2 = c - b;

        let min_moves = match (distance_1, distance_2) {
            (1, 1) => 0,
            (1 | 2, _) | (_, 1 | 2) => 1,
            _ => 2,
        };

        let max_moves = distance_1 + distance_2 - 2;

        vec![min_moves, max_moves]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        Self::num_moves_stones(a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

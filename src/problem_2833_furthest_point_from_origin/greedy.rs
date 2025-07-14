pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut slots = 0;

        let sum = moves.bytes().fold(0_i32, |sum, m| match m {
            b'L' => sum + 1,
            b'R' => sum - 1,
            _ => {
                slots += 1;

                sum
            }
        });

        (sum.unsigned_abs() + slots) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn furthest_distance_from_origin(moves: String) -> i32 {
        Self::furthest_distance_from_origin(moves)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

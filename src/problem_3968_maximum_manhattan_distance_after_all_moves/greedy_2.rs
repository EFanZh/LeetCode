pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distance(moves: String) -> i32 {
        let mut h = 0_i32;
        let mut v = 0_i32;
        let mut slots = 0;

        for m in moves.bytes() {
            let (k, v) = match m {
                b'U' => (&mut v, 1),
                b'D' => (&mut v, -1),
                b'L' => (&mut h, 1),
                b'R' => (&mut h, -1),
                _ => (&mut slots, 1),
            };

            *k += v;
        }

        h.abs() + v.abs() + slots
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(moves: String) -> i32 {
        Self::max_distance(moves)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

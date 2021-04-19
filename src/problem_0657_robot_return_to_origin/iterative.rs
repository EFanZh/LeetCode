pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;

        for c in moves.bytes() {
            match c {
                b'D' => y -= 1,
                b'L' => x -= 1,
                b'R' => x += 1,
                _ => y += 1,
            }
        }

        x == 0 && y == 0
    }
}

impl super::Solution for Solution {
    fn judge_circle(moves: String) -> bool {
        Self::judge_circle(moves)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

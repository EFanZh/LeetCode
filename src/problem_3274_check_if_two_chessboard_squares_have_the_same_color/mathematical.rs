pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        ((coordinate1.bytes().sum::<u8>() ^ coordinate2.bytes().sum::<u8>()) & 1) == 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        Self::check_two_chessboards(coordinate1, coordinate2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

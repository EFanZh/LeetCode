pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let n = n.cast_unsigned();
        let k = k.cast_unsigned();
        let mut result = 0;

        for player_count in 2..=n {
            result = (result + k) % player_count;
        }

        (result + 1).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_winner(n: i32, k: i32) -> i32 {
        Self::find_the_winner(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

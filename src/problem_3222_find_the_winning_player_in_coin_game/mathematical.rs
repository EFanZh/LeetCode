pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn winning_player(x: i32, y: i32) -> String {
        let x = x.cast_unsigned();
        let y = y.cast_unsigned();

        String::from(if x.min(y / 4).is_multiple_of(2) { "Bob" } else { "Alice" })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn winning_player(x: i32, y: i32) -> String {
        Self::winning_player(x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

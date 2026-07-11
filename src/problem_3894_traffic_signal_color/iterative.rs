pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn traffic_signal(n: i32) -> String {
        (match n {
            0 => "Green",
            30 => "Orange",
            31..=90 => "Red",
            _ => "Invalid",
        })
        .to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn traffic_signal(n: i32) -> String {
        Self::traffic_signal(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

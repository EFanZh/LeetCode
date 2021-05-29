pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| match i % 15 {
                0 => String::from("FizzBuzz"),
                3 | 6 | 9 | 12 => String::from("Fizz"),
                5 | 10 => String::from("Buzz"),
                _ => i.to_string(),
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn fizz_buzz(n: i32) -> Vec<String> {
        Self::fizz_buzz(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

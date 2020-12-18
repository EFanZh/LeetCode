pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| {
                if i % 3 == 0 {
                    if i % 5 == 0 {
                        String::from("FizzBuzz")
                    } else {
                        String::from("Fizz")
                    }
                } else if i % 5 == 0 {
                    String::from("Buzz")
                } else {
                    i.to_string()
                }
            })
            .collect()
    }
}

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

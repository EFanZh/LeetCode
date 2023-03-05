pub mod stack;

pub trait BrowserHistory {
    fn new(homepage: String) -> Self;
    fn visit(&mut self, url: String);
    fn back(&mut self, steps: i32) -> String;
    fn forward(&mut self, steps: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::BrowserHistory;

    enum Operation {
        Visit(&'static str),
        Back(i32, &'static str),
        Forward(i32, &'static str),
    }

    pub fn run<H: BrowserHistory>() {
        let test_cases = [(
            "leetcode.com",
            &[
                Operation::Visit("google.com"),
                Operation::Visit("facebook.com"),
                Operation::Visit("youtube.com"),
                Operation::Back(1, "facebook.com"),
                Operation::Back(1, "google.com"),
                Operation::Forward(1, "facebook.com"),
                Operation::Visit("linkedin.com"),
                Operation::Forward(2, "linkedin.com"),
                Operation::Back(2, "google.com"),
                Operation::Back(7, "leetcode.com"),
            ] as &[_],
        )];

        for (homepage, operations) in test_cases {
            let mut browser_history = H::new(homepage.to_string());

            for operation in operations {
                match *operation {
                    Operation::Visit(url) => browser_history.visit(url.to_string()),
                    Operation::Back(steps, expected) => assert_eq!(browser_history.back(steps), expected),
                    Operation::Forward(steps, expected) => assert_eq!(browser_history.forward(steps), expected),
                }
            }
        }
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn score_validator(events: Vec<String>) -> Vec<i32> {
        let mut score = 0;
        let mut counter = 0;

        for event in events {
            score += match event.as_str() {
                "0" => continue,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "6" => 6,
                "W" => {
                    counter += 1;

                    if counter == 10 {
                        break;
                    }

                    continue;
                }
                _ => 1,
            };
        }

        vec![score, counter]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn score_validator(events: Vec<String>) -> Vec<i32> {
        Self::score_validator(events)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn assign_umbrella(rain: Vec<i32>) -> Vec<String> {
        let mut diff = 0;

        rain.iter()
            .enumerate()
            .map(|(i, &x)| {
                String::from(if x != 0 {
                    if rain.get(i + 1).is_some_and(|&x| x != 0) && diff <= 0 {
                        diff += 1;

                        "Alice"
                    } else {
                        diff -= 1;

                        "Bob"
                    }
                } else {
                    "None"
                })
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn assign_umbrella(rain: Vec<i32>) -> Vec<String> {
        Self::assign_umbrella(rain)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub mod iterative;

pub trait Solution {
    fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("(name)is(age)yearsold", &[["name", "bob"], ["age", "two"]] as &[_]),
                "bobistwoyearsold",
            ),
            (("hi(name)", &[["a", "b"]]), "hi?"),
            (("(a)(a)(a)aaa", &[["a", "yes"]]), "yesyesyesaaa"),
        ];

        for ((s, knowledge), expected) in test_cases {
            assert_eq!(
                S::evaluate(
                    s.to_string(),
                    knowledge
                        .iter()
                        .map(|&[key, value]| vec![key.to_string(), value.to_string()])
                        .collect(),
                ),
                expected,
            );
        }
    }
}

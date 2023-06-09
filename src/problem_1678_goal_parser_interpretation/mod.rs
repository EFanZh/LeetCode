pub mod iterative;

pub trait Solution {
    fn interpret(command: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("G()(al)", "Goal"),
            ("G()()()()(al)", "Gooooal"),
            ("(al)G(al)()()G", "alGalooG"),
        ];

        for (command, expected) in test_cases {
            assert_eq!(S::interpret(command.to_string()), expected);
        }
    }
}

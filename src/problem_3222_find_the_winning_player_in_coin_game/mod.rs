pub mod mathematical;

pub trait Solution {
    fn winning_player(x: i32, y: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 7), "Alice"), ((4, 11), "Bob")];

        for ((x, y), expected) in test_cases {
            assert_eq!(S::winning_player(x, y), expected);
        }
    }
}

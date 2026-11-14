pub mod mathematical;
pub mod mathematical_2;

pub trait Solution {
    fn find_the_winner(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 2), 3), ((6, 5), 1)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::find_the_winner(n, k), expected);
        }
    }
}

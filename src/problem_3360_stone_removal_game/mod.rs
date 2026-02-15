pub mod brute_force;

pub trait Solution {
    fn can_alice_win(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        for (n, expected) in [(1, false), (12, true)] {
            assert_eq!(S::can_alice_win(n), expected);
        }
    }
}

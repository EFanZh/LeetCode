pub mod mathematical;

pub trait Solution {
    fn bulb_switch(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(0, 0), (1, 1), (2, 1), (3, 1)];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::bulb_switch(n), expected);
        }
    }
}

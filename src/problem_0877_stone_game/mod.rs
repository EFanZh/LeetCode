pub mod dynamic_programming;
pub mod mathematical;

pub trait Solution {
    fn stone_game(piles: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 3, 4, 5] as &[_], true), (&[3, 7, 2, 3], true)];

        for (piles, expected) in test_cases {
            assert_eq!(S::stone_game(piles.to_vec()), expected);
        }
    }
}

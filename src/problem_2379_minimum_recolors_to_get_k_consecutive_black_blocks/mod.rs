pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn minimum_recolors(blocks: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("WBBWWBBWBW", 7), 3), (("WBWBBBW", 2), 0)];

        for ((blocks, k), expected) in test_cases {
            assert_eq!(S::minimum_recolors(blocks.to_string(), k), expected);
        }
    }
}

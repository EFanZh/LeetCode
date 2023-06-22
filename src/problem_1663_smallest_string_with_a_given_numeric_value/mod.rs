pub mod mathematical;

pub trait Solution {
    fn get_smallest_string(n: i32, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 27), "aay"), ((5, 73), "aaszz"), ((3, 3), "aaa")];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::get_smallest_string(n, k), expected);
        }
    }
}

pub mod mathematical;

pub trait Solution {
    fn max_containers(n: i32, w: i32, max_weight: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 3, 15), 4), ((3, 5, 20), 4)];

        for ((n, w, max_weight), expected) in test_cases {
            assert_eq!(S::max_containers(n, w, max_weight), expected);
        }
    }
}

pub mod find_cycle;

pub trait Solution {
    fn smallest_repunit_div_by_k(k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (2, -1), (3, 3), (4, -1)];

        for (k, expected) in test_cases {
            assert_eq!(S::smallest_repunit_div_by_k(k), expected);
        }
    }
}

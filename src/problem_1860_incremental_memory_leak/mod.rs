pub mod simulation;

pub trait Solution {
    fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 2), [3, 1, 0]), ((8, 11), [6, 0, 4])];

        for ((memory1, memory2), expected) in test_cases {
            assert_eq!(S::mem_leak(memory1, memory2), expected);
        }
    }
}

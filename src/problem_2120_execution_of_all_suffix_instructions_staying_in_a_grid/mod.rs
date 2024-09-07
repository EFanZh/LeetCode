pub mod dequeue;

pub trait Solution {
    fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, [0, 1], "RRDDLU"), &[1, 5, 4, 3, 1, 0] as &[_]),
            ((2, [1, 1], "LURD"), &[4, 1, 0, 0]),
            ((1, [0, 0], "LRUD"), &[0, 0, 0, 0]),
        ];
        for ((n, start_pos, s), expected) in test_cases {
            assert_eq!(S::execute_instructions(n, start_pos.to_vec(), s.to_string()), expected);
        }
    }
}

pub mod dynamic_programming;

pub trait Solution {
    fn num_ways(steps: i32, arr_len: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 2), 4), ((2, 4), 2), ((4, 2), 8), ((27, 7), 127_784_505)];

        for ((steps, arr_len), expected) in test_cases {
            assert_eq!(S::num_ways(steps, arr_len), expected);
        }
    }
}

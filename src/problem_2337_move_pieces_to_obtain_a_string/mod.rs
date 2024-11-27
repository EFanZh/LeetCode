pub mod greedy;

pub trait Solution {
    fn can_change(start: String, target: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("_L__R__R_", "L______RR"), true),
            (("R_L_", "__LR"), false),
            (("_R", "R_"), false),
            (("_LL__R__R_", "L___L___RR"), false),
            (("_", "L"), false),
            (("L___L_", "_RR___"), false),
            (("_L__R__R_L", "L______RR_"), false),
        ];

        for ((start, target), expected) in test_cases {
            assert_eq!(S::can_change(start.to_string(), target.to_string()), expected);
        }
    }
}

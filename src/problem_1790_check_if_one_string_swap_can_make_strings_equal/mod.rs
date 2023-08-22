pub mod iterative;

pub trait Solution {
    fn are_almost_equal(s1: String, s2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("bank", "kanb"), true),
            (("attack", "defend"), false),
            (("kelb", "kelb"), true),
            (("aa", "ac"), false),
            (
                (
                    "siyolsdcjthwsiplccjbuceoxmpjgrauocx",
                    "siyolsdcjthwsiplccpbuceoxmjjgrauocx",
                ),
                true,
            ),
        ];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::are_almost_equal(s1.to_string(), s2.to_string()), expected);
        }
    }
}

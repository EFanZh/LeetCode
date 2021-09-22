pub mod iterative;

pub trait Solution {
    fn can_transform(start: String, end: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("RXXLRXRXL", "XRLXXRRLX"), true),
            (("X", "L"), false),
            (("LLR", "RRL"), false),
            (("XL", "LX"), true),
            (("XLLR", "LXLX"), false),
            (("LXXLXRLXXL", "XLLXRXLXLX"), false),
            (("XRRXRX", "RXLRRX"), false),
        ];

        for ((start, end), expected) in test_cases {
            assert_eq!(S::can_transform(start.to_string(), end.to_string()), expected);
        }
    }
}

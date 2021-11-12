pub mod iterative;

pub trait Solution {
    fn push_dominoes(dominoes: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("RR.L", "RR.L"), (".L.R...LR..L..", "LL.RR.LLRRLL..")];

        for (dominoes, expected) in test_cases {
            assert_eq!(S::push_dominoes(dominoes.to_string()), expected);
        }
    }
}

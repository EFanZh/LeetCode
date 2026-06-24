pub mod iterative;

pub trait Solution {
    fn reverse_by_type(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(")ebc#da@f(", "(fad@cb#e)"), ("z", "z"), ("!@#$%^&*()", ")(*&^%$#@!")];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_by_type(s.to_string()), expected);
        }
    }
}

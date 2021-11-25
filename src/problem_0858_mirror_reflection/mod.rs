pub mod mathematical;
pub mod mathematical_2;

pub trait Solution {
    fn mirror_reflection(p: i32, q: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 1), 2), ((3, 1), 1), ((3, 2), 0), ((5, 3), 1)];

        for ((p, q), expected) in test_cases {
            assert_eq!(S::mirror_reflection(p, q), expected);
        }
    }
}

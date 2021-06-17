pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn length_longest_path(input: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext", 20),
            (
                "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext",
                32,
            ),
            ("a", 0),
            ("file1.txt\nfile2.txt\nlongfile.txt", 12),
        ];

        for (input, expected) in test_cases {
            assert_eq!(S::length_longest_path(input.to_string()), expected);
        }
    }
}

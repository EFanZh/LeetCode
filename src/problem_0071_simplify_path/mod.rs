pub mod stack;

pub trait Solution {
    fn simplify_path(path: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("/home/", "/home"),
            ("/../", "/"),
            ("/home//foo/", "/home/foo"),
            ("/a/./b/../../c/", "/c"),
            ("/a/../../b/../c//.//", "/c"),
            ("/a//b////c/d//././/..", "/a/b/c"),
        ];

        for (path, expected) in test_cases.iter().copied() {
            assert_eq!(S::simplify_path(path.to_string()), expected);
        }
    }
}

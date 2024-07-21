pub mod greedy;

pub trait Solution {
    fn check_palindrome_formation(a: String, b: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("x", "y"), true),
            (("xbdef", "xecab"), false),
            (("ulacfd", "jizalu"), true),
            (("abda", "acmc"), false),
            (
                (
                    "aejbaalflrmkswrydwdkdwdyrwskmrlfqizjezd",
                    "uvebspqckawkhbrtlqwblfwzfptanhiglaabjea",
                ),
                true,
            ),
            (("abccef", "cgdhga"), false),
            (
                (
                    "alcojrucnipkxjrwvuvnwraixyndxzfbwgyuaohesyciodlukfbbqkprcwkxumumuxkwcrpkqbbfkuldoicysehoauygwbfzxdnyxiarwnvuvwrrjcyrltbetaahm",
                    "mhaatebtlrycjrvzmboowswllwvupbbcxplmrtlhpcnkulvmaaroopdklzfaegvagiknaowcjkemvaiaysmrcudmmbofzjccrixztupmzuvupzjpoloebmfmeoeqo",
                ),
                true,
            ),
        ];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::check_palindrome_formation(a.to_string(), b.to_string()), expected);
        }
    }
}

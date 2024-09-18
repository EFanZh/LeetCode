pub mod greedy_stack;

pub trait Solution {
    fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("leet", 3, 'e', 1), "eet"),
            (("leetcode", 4, 'e', 2), "ecde"),
            (("bb", 2, 'b', 2), "bb"),
            (("aaabbbcccddd", 3, 'b', 2), "abb"),
            (("facfffkfnffoppfffzfz", 9, 'f', 9), "fffffffff"),
            (
                (
                    "xxxxvvuuosqppppooommmlkjjoihgffddcccbbacdefijlommnnopprtvvwxxyyz",
                    8,
                    'o',
                    3,
                ),
                "jjoacdoo",
            ),
        ];

        for ((s, k, letter, repetition), expected) in test_cases {
            assert_eq!(S::smallest_subsequence(s.to_string(), k, letter, repetition), expected);
        }
    }
}

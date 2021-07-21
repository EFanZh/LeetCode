pub mod quick_select;

pub trait Solution {
    fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    type TestCase = ((&'static [&'static str], i32), &'static [&'static str]);

    const TEST_CASES: &[TestCase] = &[
        (
            (&["i", "love", "leetcode", "i", "love", "coding"] as &[_], 2),
            &["i", "love"] as &[_],
        ),
        (
            (
                &["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"],
                4,
            ),
            &["the", "is", "sunny", "day"],
        ),
        ((&["a", "aa", "aaa"], 1), &["a"]),
        (
            (
                &[
                    "plpaboutit",
                    "jnoqzdute",
                    "sfvkdqf",
                    "mjc",
                    "nkpllqzjzp",
                    "foqqenbey",
                    "ssnanizsav",
                    "nkpllqzjzp",
                    "sfvkdqf",
                    "isnjmy",
                    "pnqsz",
                    "hhqpvvt",
                    "fvvdtpnzx",
                    "jkqonvenhx",
                    "cyxwlef",
                    "hhqpvvt",
                    "fvvdtpnzx",
                    "plpaboutit",
                    "sfvkdqf",
                    "mjc",
                    "fvvdtpnzx",
                    "bwumsj",
                    "foqqenbey",
                    "isnjmy",
                    "nkpllqzjzp",
                    "hhqpvvt",
                    "foqqenbey",
                    "fvvdtpnzx",
                    "bwumsj",
                    "hhqpvvt",
                    "fvvdtpnzx",
                    "jkqonvenhx",
                    "jnoqzdute",
                    "foqqenbey",
                    "jnoqzdute",
                    "foqqenbey",
                    "hhqpvvt",
                    "ssnanizsav",
                    "mjc",
                    "foqqenbey",
                    "bwumsj",
                    "ssnanizsav",
                    "fvvdtpnzx",
                    "nkpllqzjzp",
                    "jkqonvenhx",
                    "hhqpvvt",
                    "mjc",
                    "isnjmy",
                    "bwumsj",
                    "pnqsz",
                    "hhqpvvt",
                    "nkpllqzjzp",
                    "jnoqzdute",
                    "pnqsz",
                    "nkpllqzjzp",
                    "jnoqzdute",
                    "foqqenbey",
                    "nkpllqzjzp",
                    "hhqpvvt",
                    "fvvdtpnzx",
                    "plpaboutit",
                    "jnoqzdute",
                    "sfvkdqf",
                    "fvvdtpnzx",
                    "jkqonvenhx",
                    "jnoqzdute",
                    "nkpllqzjzp",
                    "jnoqzdute",
                    "fvvdtpnzx",
                    "jkqonvenhx",
                    "hhqpvvt",
                    "isnjmy",
                    "jkqonvenhx",
                    "ssnanizsav",
                    "jnoqzdute",
                    "jkqonvenhx",
                    "fvvdtpnzx",
                    "hhqpvvt",
                    "bwumsj",
                    "nkpllqzjzp",
                    "bwumsj",
                    "jkqonvenhx",
                    "jnoqzdute",
                    "pnqsz",
                    "foqqenbey",
                    "sfvkdqf",
                    "sfvkdqf",
                ],
                1,
            ),
            &["fvvdtpnzx"],
        ),
    ];

    pub fn run<S: Solution>() {
        for ((words, k), expected) in TEST_CASES.iter().copied() {
            assert_eq!(
                S::top_k_frequent(words.iter().copied().map(str::to_string).collect(), k),
                expected
            );
        }
    }
}

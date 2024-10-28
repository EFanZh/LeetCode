pub mod hash_map;

pub trait Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self;
    fn encrypt(&self, word1: String) -> String;
    fn decrypt(&self, word2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Encrypter;

    enum Operation {
        Encrypt(&'static str, &'static str),
        Decrypt(&'static str, i32),
    }

    pub fn run<E: Encrypter>() {
        let test_cases = [
            (
                (
                    "abcd",
                    &["ei", "zf", "ei", "am"] as &[_],
                    &["abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad"] as &[_],
                ),
                &[
                    Operation::Encrypt("abcd", "eizfeiam"),
                    Operation::Decrypt("eizfeiam", 2),
                ] as &[_],
            ),
            (
                ("b", &["ca"], &["aaa", "cacbc", "bbaba", "bb"]),
                &[Operation::Encrypt("bbb", "cacaca"), Operation::Decrypt("cacaca", 0)],
            ),
        ];

        for ((keys, values, dictionary), operations) in test_cases {
            let encrypter = E::new(
                keys.chars().collect(),
                values.iter().copied().map(str::to_string).collect(),
                dictionary.iter().copied().map(str::to_string).collect(),
            );

            for operation in operations {
                match *operation {
                    Operation::Encrypt(word1, expected) => assert_eq!(encrypter.encrypt(word1.to_string()), expected),
                    Operation::Decrypt(word2, expected) => assert_eq!(encrypter.decrypt(word2.to_string()), expected),
                }
            }
        }
    }
}

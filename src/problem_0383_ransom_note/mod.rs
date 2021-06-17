pub mod direct_address_table;
pub mod hash_map;

pub trait Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("a", "b"), false), (("aa", "ab"), false), (("aa", "aab"), true)];

        for ((ransom_note, magazine), expected) in test_cases {
            assert_eq!(
                S::can_construct(ransom_note.to_string(), magazine.to_string()),
                expected
            );
        }
    }
}

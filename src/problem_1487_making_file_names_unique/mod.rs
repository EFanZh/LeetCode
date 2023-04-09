pub mod hash_map;

pub trait Solution {
    fn get_folder_names(names: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &["pes", "fifa", "gta", "pes(2019)"] as &[_],
                &["pes", "fifa", "gta", "pes(2019)"] as &[_],
            ),
            (
                &["gta", "gta(1)", "gta", "avalon"],
                &["gta", "gta(1)", "gta(2)", "avalon"],
            ),
            (
                &["onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece"],
                &["onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece(4)"],
            ),
            (
                &[
                    "r", "y", "m", "o(3)(2)", "f", "r", "z", "u", "w", "q(2)(3)", "a", "s", "k", "o", "y", "b", "n",
                    "t(2)(4)", "s", "e", "r", "v", "g", "q(1)(4)", "j", "j", "r(4)(4)", "t",
                ],
                &[
                    "r", "y", "m", "o(3)(2)", "f", "r(1)", "z", "u", "w", "q(2)(3)", "a", "s", "k", "o", "y(1)", "b",
                    "n", "t(2)(4)", "s(1)", "e", "r(2)", "v", "g", "q(1)(4)", "j", "j(1)", "r(4)(4)", "t",
                ],
            ),
        ];

        for (names, expected) in test_cases {
            assert_eq!(
                S::get_folder_names(names.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}

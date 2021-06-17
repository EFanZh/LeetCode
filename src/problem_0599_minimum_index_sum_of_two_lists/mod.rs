pub mod index_map;

pub trait Solution {
    fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["Shogun", "Tapioca Express", "Burger King", "KFC"] as &[_],
                    &[
                        "Piatti",
                        "The Grill at Torrey Pines",
                        "Hungry Hunter Steakhouse",
                        "Shogun",
                    ] as &[_],
                ),
                &["Shogun"] as &[_],
            ),
            (
                (
                    &["Shogun", "Tapioca Express", "Burger King", "KFC"],
                    &["KFC", "Shogun", "Burger King"],
                ),
                &["Shogun"],
            ),
            (
                (
                    &["Shogun", "Tapioca Express", "Burger King", "KFC"],
                    &["KFC", "Burger King", "Tapioca Express", "Shogun"],
                ),
                &["KFC", "Burger King", "Tapioca Express", "Shogun"],
            ),
            (
                (
                    &["Shogun", "Tapioca Express", "Burger King", "KFC"],
                    &["KNN", "KFC", "Burger King", "Tapioca Express", "Shogun"],
                ),
                &["KFC", "Burger King", "Tapioca Express", "Shogun"],
            ),
            ((&["KFC"], &["KFC"]), &["KFC"]),
            ((&["S", "TEXP", "BK", "KFC"], &["KFC", "BK", "S"]), &["S"]),
        ];

        for ((list1, list2), expected) in test_cases {
            assert_eq!(
                S::find_restaurant(
                    list1.iter().copied().map(str::to_string).collect(),
                    list2.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}

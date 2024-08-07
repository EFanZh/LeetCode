pub mod bfs;

pub trait Solution {
    fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["bread"] as &[_],
                    &[&["yeast", "flour"] as &[_]] as &[&[_]],
                    &["yeast", "flour", "corn"] as &[_],
                ),
                &["bread"] as &[_],
            ),
            (
                (
                    &["bread", "sandwich"],
                    &[&["yeast", "flour"], &["bread", "meat"]],
                    &["yeast", "flour", "meat"],
                ),
                &["bread", "sandwich"],
            ),
            (
                (
                    &["bread", "sandwich", "burger"],
                    &[&["yeast", "flour"], &["bread", "meat"], &["sandwich", "meat", "bread"]],
                    &["yeast", "flour", "meat"],
                ),
                &["bread", "burger", "sandwich"],
            ),
            (
                (
                    &["ju", "fzjnm", "x", "e", "zpmcz", "h", "q"],
                    &[
                        &["d"],
                        &["hveml", "f", "cpivl"],
                        &["cpivl", "zpmcz", "h", "e", "fzjnm", "ju"],
                        &["cpivl", "hveml", "zpmcz", "ju", "h"],
                        &["h", "fzjnm", "e", "q", "x"],
                        &["d", "hveml", "cpivl", "q", "zpmcz", "ju", "e", "x"],
                        &["f", "hveml", "cpivl"],
                    ],
                    &["f", "hveml", "cpivl", "d"],
                ),
                &["fzjnm", "ju", "q"],
            ),
        ];

        for ((recipes, ingredients, supplies), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_all_recipes(
                    recipes.iter().copied().map(str::to_string).collect(),
                    ingredients
                        .iter()
                        .map(|ingredients| ingredients.iter().copied().map(str::to_string).collect())
                        .collect(),
                    supplies.iter().copied().map(str::to_string).collect(),
                )),
                expected,
            );
        }
    }
}

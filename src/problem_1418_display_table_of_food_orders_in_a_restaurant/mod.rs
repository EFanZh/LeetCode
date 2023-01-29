pub mod hash_map;

pub trait Solution {
    fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    ["David", "3", "Ceviche"],
                    ["Corina", "10", "Beef Burrito"],
                    ["David", "3", "Fried Chicken"],
                    ["Carla", "5", "Water"],
                    ["Carla", "5", "Ceviche"],
                    ["Rous", "3", "Ceviche"],
                ] as &[_],
                &[
                    ["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
                    ["3", "0", "2", "1", "0"],
                    ["5", "0", "1", "0", "1"],
                    ["10", "1", "0", "0", "0"],
                ] as &dyn Matrix<_>,
            ),
            (
                &[
                    ["James", "12", "Fried Chicken"],
                    ["Ratesh", "12", "Fried Chicken"],
                    ["Amadeus", "12", "Fried Chicken"],
                    ["Adam", "1", "Canadian Waffles"],
                    ["Brianna", "1", "Canadian Waffles"],
                ],
                &[
                    ["Table", "Canadian Waffles", "Fried Chicken"],
                    ["1", "2", "0"],
                    ["12", "0", "3"],
                ],
            ),
            (
                &[
                    ["Laura", "2", "Bean Burrito"],
                    ["Jhon", "2", "Beef Burrito"],
                    ["Melissa", "2", "Soda"],
                ],
                &[["Table", "Bean Burrito", "Beef Burrito", "Soda"], ["2", "1", "1", "1"]],
            ),
        ];

        for (accounts, expected) in test_cases {
            assert_eq!(
                S::display_table(
                    accounts
                        .iter()
                        .map(|orders| orders.iter().copied().map(str::to_string).collect())
                        .collect(),
                ),
                expected.to_vec(),
            );
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        [1, 4, 1, 40, 10],
                        [2, 8, 0, 50, 5],
                        [3, 8, 1, 30, 4],
                        [4, 10, 0, 10, 3],
                        [5, 1, 1, 15, 1],
                    ] as &[_],
                    1,
                    50,
                    10,
                ),
                &[3, 1, 5] as &[_],
            ),
            (
                (
                    &[
                        [1, 4, 1, 40, 10],
                        [2, 8, 0, 50, 5],
                        [3, 8, 1, 30, 4],
                        [4, 10, 0, 10, 3],
                        [5, 1, 1, 15, 1],
                    ],
                    0,
                    50,
                    10,
                ),
                &[4, 3, 2, 1, 5],
            ),
            (
                (
                    &[
                        [1, 4, 1, 40, 10],
                        [2, 8, 0, 50, 5],
                        [3, 8, 1, 30, 4],
                        [4, 10, 0, 10, 3],
                        [5, 1, 1, 15, 1],
                    ],
                    0,
                    30,
                    3,
                ),
                &[4, 5],
            ),
        ];

        for ((restaurants, vegan_friendly, max_price, max_distance), expected) in test_cases {
            assert_eq!(
                S::filter_restaurants(
                    restaurants.iter().map(Vec::from).collect(),
                    vegan_friendly,
                    max_price,
                    max_distance
                ),
                expected
            );
        }
    }
}

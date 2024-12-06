pub mod binary_heap;

pub trait FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self;
    fn change_rating(&mut self, food: String, new_rating: i32);
    fn highest_rated(&self, cuisine: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::FoodRatings;

    enum Operation {
        ChangeRating(&'static str, i32),
        HighestRated(&'static str, &'static str),
    }

    pub fn run<F: FoodRatings>() {
        let test_cases = [
            (
                (
                    &["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"] as &[_],
                    &["korean", "japanese", "japanese", "greek", "japanese", "korean"] as &[_],
                    &[9, 12, 8, 15, 14, 7] as &[_],
                ),
                &[
                    Operation::HighestRated("korean", "kimchi"),
                    Operation::HighestRated("japanese", "ramen"),
                    Operation::ChangeRating("sushi", 16),
                    Operation::HighestRated("japanese", "sushi"),
                    Operation::ChangeRating("ramen", 16),
                    Operation::HighestRated("japanese", "ramen"),
                ] as &[_],
            ),
            (
                (
                    &["emgqdbo", "jmvfxjohq", "qnvseohnoe", "yhptazyko", "ocqmvmwjq"],
                    &["snaxol", "snaxol", "snaxol", "fajbervsj", "fajbervsj"],
                    &[2, 6, 18, 6, 5],
                ),
                &[
                    Operation::ChangeRating("qnvseohnoe", 11),
                    Operation::HighestRated("fajbervsj", "yhptazyko"),
                    Operation::ChangeRating("emgqdbo", 3),
                    Operation::ChangeRating("jmvfxjohq", 9),
                    Operation::ChangeRating("emgqdbo", 14),
                    Operation::HighestRated("fajbervsj", "yhptazyko"),
                    Operation::HighestRated("snaxol", "emgqdbo"),
                ],
            ),
            (
                (
                    &["xucxenyckh", "rmzka", "kiesprtv"],
                    &["oqinnmfsaf", "tgeb", "onzfzqjw"],
                    &[8, 4, 3],
                ),
                &[
                    Operation::ChangeRating("rmzka", 8),
                    Operation::ChangeRating("rmzka", 8),
                    Operation::ChangeRating("rmzka", 5),
                    Operation::HighestRated("onzfzqjw", "kiesprtv"),
                    Operation::HighestRated("oqinnmfsaf", "xucxenyckh"),
                    Operation::HighestRated("onzfzqjw", "kiesprtv"),
                ],
            ),
        ];

        for ((foods, cuisines, ratings), operations) in test_cases {
            let mut food_ratings = F::new(
                foods.iter().copied().map(str::to_string).collect(),
                cuisines.iter().copied().map(str::to_string).collect(),
                ratings.to_vec(),
            );

            for operation in operations {
                match *operation {
                    Operation::ChangeRating(food, new_rating) => {
                        food_ratings.change_rating(food.to_string(), new_rating);
                    }
                    Operation::HighestRated(cuisine, expected) => {
                        assert_eq!(food_ratings.highest_rated(cuisine.to_string()), expected.to_string());
                    }
                }
            }
        }
    }
}

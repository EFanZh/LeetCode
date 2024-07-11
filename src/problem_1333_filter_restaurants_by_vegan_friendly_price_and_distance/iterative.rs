pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    fn unwrap_restaurant(restaurant: &[i32]) -> [i32; 5] {
        restaurant.try_into().ok().unwrap()
    }

    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut restaurants = restaurants;

        if vegan_friendly == 0 {
            restaurants.retain(|restaurant| {
                let [_, _, _, price, distance]: [_; 5] = Self::unwrap_restaurant(restaurant);

                price <= max_price && distance <= max_distance
            });
        } else {
            restaurants.retain(|restaurant| {
                let [_, _, vegan_friendly, price, distance]: [_; 5] = Self::unwrap_restaurant(restaurant);

                vegan_friendly != 0 && price <= max_price && distance <= max_distance
            });
        }

        restaurants.sort_unstable_by_key(|restaurant| {
            let [id, rating, _, _, _]: [_; 5] = Self::unwrap_restaurant(restaurant);

            Reverse((rating, id))
        });

        restaurants.into_iter().map(|restaurant| restaurant[0]).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        Self::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

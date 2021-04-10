pub struct Solution;

impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort_unstable();
        heaters.sort_unstable();

        let mut result = 0;
        let mut house_iter = houses.iter().copied();
        let mut house = house_iter.next().unwrap();

        for (heater, boundary) in heaters
            .iter()
            .zip(&heaters[1..])
            .map(|(&left, &right)| (left, (left + right) / 2))
        {
            while house <= boundary {
                result = result.max((house - heater).abs());

                if let Some(next_house) = house_iter.next() {
                    house = next_house;
                } else {
                    return result;
                }
            }
        }

        result = result.max((house - heaters.last().unwrap()).abs());
        result = result.max((houses.last().unwrap() - heaters.last().unwrap()).abs());

        result
    }
}

impl super::Solution for Solution {
    fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        Self::find_radius(houses, heaters)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

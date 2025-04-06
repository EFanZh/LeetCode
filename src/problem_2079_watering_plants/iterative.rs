pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut available = capacity;
        let mut result = plants.len() as i32;

        for (double_x, required) in (0..).step_by(2).zip(plants) {
            if available < required {
                result += double_x;
                available = capacity;
            }

            available -= required;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        Self::watering_plants(plants, capacity)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

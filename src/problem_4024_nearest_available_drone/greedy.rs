pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
        let [target_x, target_y] = target.try_into().ok().unwrap();
        let mut result = -1;
        let mut min_distance = u32::MAX;

        (0..).zip(drones).for_each(|(i, drone)| {
            let [drone_x, drone_y, range] = drone.try_into().ok().unwrap();
            let distance = drone_x.abs_diff(target_x) + drone_y.abs_diff(target_y);

            if distance < min_distance && distance <= range.cast_unsigned() {
                result = i;
                min_distance = distance;
            }
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
        Self::nearest_drone(drones, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

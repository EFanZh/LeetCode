pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/car-fleet-ii/discuss/1085987/JavaC%2B%2BPython-O(n)-Stack-Solution>.

impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let mut result = vec![-1.0; cars.len()];
        let mut stack = Vec::<(u32, u32, u32, u32)>::new();

        for (target, car) in result.iter_mut().zip(cars).rev() {
            let [position, speed] = car.try_into().ok().unwrap();
            let (position, speed) = (position as u32, speed as u32);

            let (distance, speed_diff) = loop {
                if let Some(&(right_position, right_speed, numerator, denominator)) = stack.last() {
                    if speed > right_speed {
                        let distance = right_position - position;
                        let speed_diff = speed - right_speed;

                        if u64::from(distance) * u64::from(denominator) < u64::from(numerator) * u64::from(speed_diff) {
                            *target = f64::from(distance) / f64::from(speed_diff);

                            break (distance, speed_diff);
                        }
                    }

                    stack.pop();
                } else {
                    break (u32::MAX, 0);
                }
            };

            stack.push((position, speed, distance, speed_diff));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        Self::get_collision_times(cars)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

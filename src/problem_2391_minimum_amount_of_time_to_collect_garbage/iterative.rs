pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut total_pickup_time = 0;
        let mut total_metal_travel_time = 0;
        let mut total_plastic_travel_time = 0;
        let mut total_glass_travel_time = 0;
        let mut total_travel_time = 0;

        garbage
            .iter()
            .zip(iter::once(0).chain(travel))
            .for_each(|(garbage, travel_cost)| {
                total_pickup_time += garbage.len() as i32;

                let mut state = 0;

                for c in garbage.bytes() {
                    state |= match c {
                        b'M' => 1,
                        b'P' => 2,
                        _ => 4,
                    };

                    if state == 7 {
                        break;
                    }
                }

                total_travel_time += travel_cost;

                if state & 1 != 0 {
                    total_metal_travel_time = total_travel_time;
                }

                if state & 2 != 0 {
                    total_plastic_travel_time = total_travel_time;
                }

                if state & 4 != 0 {
                    total_glass_travel_time = total_travel_time;
                }
            });

        total_pickup_time + total_metal_travel_time + total_plastic_travel_time + total_glass_travel_time
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        Self::garbage_collection(garbage, travel)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

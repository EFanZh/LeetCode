pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut events = Vec::with_capacity(trips.len() * 2);

        for trip in &trips {
            let [passengers, from, to]: [_; 3] = trip.as_slice().try_into().unwrap();

            events.extend([Reverse((from as u32, passengers)), Reverse((to as u32, -passengers))]);
        }

        drop(trips);

        let mut events = BinaryHeap::from(events);
        let mut capacity = capacity;

        while let Some(Reverse((_, passengers))) = events.pop() {
            capacity -= passengers;

            if capacity < 0 {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        Self::car_pooling(trips, capacity)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

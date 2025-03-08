pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut events = Vec::with_capacity(trips.len() * 2);

        for trip in &trips {
            let [passengers, from, to] = trip.as_slice().try_into().unwrap();

            events.extend([(from as u32, passengers), (to as u32, -passengers)]);
        }

        drop(trips);

        events.sort_unstable();

        let mut current = 0;

        for (_, passengers) in events {
            current += passengers;

            if current > capacity {
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

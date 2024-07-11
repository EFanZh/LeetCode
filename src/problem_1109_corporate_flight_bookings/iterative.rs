pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut result = vec![0; n as _];

        for booking in bookings {
            let [first, last, seats]: [_; 3] = booking.try_into().unwrap();

            result[(first as usize) - 1] += seats;

            if let Some(value) = result.get_mut(last as usize) {
                *value -= seats;
            }
        }

        let mut sum = 0;

        for value in &mut result {
            sum += *value;
            *value = sum;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        Self::corp_flight_bookings(bookings, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

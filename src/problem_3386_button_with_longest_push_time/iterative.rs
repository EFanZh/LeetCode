pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut max_index = 0;
        let mut max_duration = 0;
        let mut prev_index = 0;
        let mut prev_time = 0;

        events
            .iter()
            .filter_map(|event| <[i32; 2]>::try_from(event.as_slice()).ok())
            .map(|event| event.map(i32::cast_unsigned))
            .for_each(|[index, time]| {
                let duration = time - prev_time;

                match duration.cmp(&max_duration) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        if index < max_index {
                            max_index = index;
                        }
                    }
                    Ordering::Greater => {
                        max_duration = duration;
                        max_index = index;
                    }
                }

                prev_index = index;
                prev_time = time;
            });

        max_index.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        Self::button_with_longest_time(events)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

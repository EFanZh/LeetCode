pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

struct Event<'a> {
    time_and_type: u32,
    data: Option<&'a mut i32>,
}

impl PartialEq for Event<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Event<'_> {}

impl PartialOrd for Event<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_and_type.cmp(&other.time_and_type)
    }
}

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut result = people;

        let mut events = flowers
            .into_iter()
            .flat_map(|flower| {
                let [start, end] = <[_; 2]>::map(flower.try_into().ok().unwrap(), |x| x as u32);

                [
                    Event {
                        time_and_type: start * 4,
                        data: None,
                    },
                    Event {
                        time_and_type: end * 4 + 3,
                        data: None,
                    },
                ]
            })
            .chain(result.iter_mut().map(|people| Event {
                time_and_type: *people as u32 * 4 + 1,
                data: Some(people),
            }))
            .collect::<Vec<_>>();

        events.sort_unstable();

        let mut count = 0;

        for Event { time_and_type, data } in events {
            if let Some(target) = data {
                *target = count;
            } else {
                count += 1 - ((time_and_type as i32 & 1) << 1);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        Self::full_bloom_flowers(flowers, people)
    }
}

#[cfg(test)]
mod tests {
    use super::Event;

    #[test]
    fn test_event_partial_eq() {
        assert!(
            Event {
                time_and_type: 2,
                data: None,
            } == Event {
                time_and_type: 2,
                data: Some(&mut 4),
            },
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

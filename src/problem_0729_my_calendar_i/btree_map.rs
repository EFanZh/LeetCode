pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BTreeMap;

struct MyCalendar {
    ranges: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn book(&mut self, left: i32, right: i32) -> bool {
        let mut left = left;
        let mut right = right;
        let mut iter = self.ranges.range(..=right).map(|(&k, &v)| (k, v));

        if let Some((last_left, last_right)) = iter.next_back() {
            match last_right.cmp(&left) {
                Ordering::Less => {}
                Ordering::Equal => left = last_left,
                Ordering::Greater => {
                    if last_left == right {
                        if let Some((next_left, next_right)) = iter.next_back() {
                            match next_right.cmp(&left) {
                                Ordering::Less => {}
                                Ordering::Equal => left = next_left,
                                Ordering::Greater => return false,
                            }
                        }

                        self.ranges.remove(&last_left);

                        right = last_right;
                    } else {
                        return false;
                    }
                }
            }
        }

        self.ranges.insert(left, right);

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MyCalendar for MyCalendar {
    fn new() -> Self {
        Self::new()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        self.book(start, end)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyCalendar>();
    }
}

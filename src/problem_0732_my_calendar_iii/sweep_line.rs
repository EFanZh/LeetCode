// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BTreeMap;

pub struct MyCalendarThree {
    events: BTreeMap<i32, i32>,
    max_booking: i32,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self {
            events: BTreeMap::new(),
            max_booking: 0,
        }
    }

    fn book(&mut self, left: i32, right: i32) -> i32 {
        self.events.entry(left).and_modify(|event| *event += 1).or_insert(1);
        self.events.entry(right).and_modify(|event| *event -= 1).or_insert(-1);

        let mut max_count = 0;
        let mut count = 0;

        for (&x, &event) in &self.events {
            if x < right {
                count += event;
                max_count = max_count.max(count);
            } else {
                break;
            }
        }

        self.max_booking = max_count.max(self.max_booking);

        self.max_booking
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MyCalendarThree for MyCalendarThree {
    fn new() -> Self {
        Self::new()
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.book(start, end)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyCalendarThree>();
    }
}

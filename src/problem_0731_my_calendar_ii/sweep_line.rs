// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BTreeMap;

pub struct MyCalendarTwo {
    events: BTreeMap<i32, i8>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }

    fn book(&mut self, left: i32, right: i32) -> bool {
        let mut iter = self.events.iter().map(|(&k, &v)| (k, v));
        let mut count = 0;

        while let Some((mut x, mut event)) = iter.next() {
            if x <= left {
                count += event;
            } else {
                count += 1;

                if count == 3 {
                    return false;
                }

                while x < right {
                    count += event;

                    if count == 3 {
                        return false;
                    }

                    if let Some((next_x, next_event)) = iter.next() {
                        x = next_x;
                        event = next_event;
                    } else {
                        break;
                    }
                }

                break;
            }
        }

        self.events.entry(left).and_modify(|event| *event += 1).or_insert(1);
        self.events.entry(right).and_modify(|event| *event -= 1).or_insert(-1);

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MyCalendarTwo for MyCalendarTwo {
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
        super::super::tests::run::<super::MyCalendarTwo>();
    }
}

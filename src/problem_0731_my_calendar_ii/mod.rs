pub mod sweep_line;

pub trait MyCalendarTwo {
    fn new() -> Self;
    fn book(&mut self, start: i32, end: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MyCalendarTwo;

    pub fn run<C: MyCalendarTwo>() {
        let test_cases = [
            &[
                ((10, 20), true),
                ((50, 60), true),
                ((10, 40), true),
                ((5, 15), false),
                ((5, 10), true),
                ((25, 55), true),
            ] as &[_],
            &[
                ((24, 40), true),
                ((43, 50), true),
                ((27, 43), true),
                ((5, 21), true),
                ((30, 40), false),
                ((14, 29), false),
                ((3, 19), true),
                ((3, 14), false),
                ((25, 39), false),
                ((6, 19), false),
            ],
            &[
                ((28, 46), true),
                ((9, 21), true),
                ((21, 39), true),
                ((37, 48), false),
                ((38, 50), false),
                ((22, 39), false),
                ((45, 50), true),
                ((1, 12), true),
                ((40, 50), false),
                ((31, 44), false),
            ],
        ];

        for books in test_cases {
            let mut my_calendar = C::new();

            for &((start, end), expected) in books {
                assert_eq!(my_calendar.book(start, end), expected);
            }
        }
    }
}

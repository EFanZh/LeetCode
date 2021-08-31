pub mod sweep_line;

pub trait MyCalendarThree {
    fn new() -> Self;
    fn book(&mut self, start: i32, end: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::MyCalendarThree;

    pub fn run<C: MyCalendarThree>() {
        let test_cases = [&[
            ((10, 20), 1),
            ((50, 60), 1),
            ((10, 40), 2),
            ((5, 15), 3),
            ((5, 10), 3),
            ((25, 55), 3),
        ] as &[_]];

        for books in test_cases {
            let mut my_calendar = C::new();

            for &((start, end), expected) in books {
                assert_eq!(my_calendar.book(start, end), expected);
            }
        }
    }
}

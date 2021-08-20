pub mod btree_map;

pub trait MyCalendar {
    fn new() -> Self;
    fn book(&mut self, start: i32, end: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MyCalendar;

    pub fn run<C: MyCalendar>() {
        let test_cases = [
            &[((10, 20), true), ((15, 25), false), ((20, 30), true)] as &[_],
            &[
                ((47, 50), true),
                ((33, 41), true),
                ((39, 45), false),
                ((33, 42), false),
                ((25, 32), true),
                ((26, 35), false),
                ((19, 25), true),
                ((3, 8), true),
                ((8, 13), true),
                ((18, 27), false),
            ],
            &[
                ((20, 29), true),
                ((13, 22), false),
                ((44, 50), true),
                ((1, 7), true),
                ((2, 10), false),
                ((14, 20), true),
                ((19, 25), false),
                ((36, 42), true),
                ((45, 50), false),
                ((47, 50), false),
                ((39, 45), false),
                ((44, 50), false),
                ((16, 25), false),
                ((45, 50), false),
                ((45, 50), false),
                ((12, 20), false),
                ((21, 29), false),
                ((11, 20), false),
                ((12, 17), false),
                ((34, 40), false),
                ((10, 18), false),
                ((38, 44), false),
                ((23, 32), false),
                ((38, 44), false),
                ((15, 20), false),
                ((27, 33), false),
                ((34, 42), false),
                ((44, 50), false),
                ((35, 40), false),
                ((24, 31), false),
            ],
            &[
                ((69, 70), true),
                ((3, 4), true),
                ((39, 40), true),
                ((35, 36), true),
                ((3, 4), false),
                ((55, 56), true),
                ((61, 62), true),
                ((97, 98), true),
                ((79, 80), true),
                ((76, 77), true),
                ((46, 47), true),
                ((78, 79), true),
                ((47, 48), true),
                ((38, 39), true),
                ((83, 84), true),
                ((90, 91), true),
                ((90, 91), false),
                ((49, 50), true),
                ((49, 50), false),
                ((77, 78), true),
                ((23, 24), true),
                ((89, 90), true),
                ((8, 9), true),
                ((3, 4), false),
                ((2, 3), true),
                ((48, 49), true),
                ((96, 97), true),
                ((4, 5), true),
                ((54, 55), true),
                ((30, 31), true),
                ((97, 98), false),
                ((65, 66), true),
                ((93, 94), true),
                ((49, 50), false),
                ((24, 25), true),
                ((17, 18), true),
                ((53, 54), true),
                ((45, 46), true),
                ((53, 54), false),
                ((32, 33), true),
                ((37, 38), true),
                ((5, 6), true),
                ((50, 51), true),
                ((48, 49), false),
                ((14, 15), true),
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

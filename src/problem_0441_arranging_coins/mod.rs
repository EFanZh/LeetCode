pub mod newtons_method;
pub mod smart_newtons_method;
pub mod smarter_newtons_method;

pub trait Solution {
    fn arrange_coins(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 3),
            (10, 4),
            (11, 4),
            (12, 4),
            (13, 4),
            (14, 4),
            (15, 5),
            (8254, 127),
            (0x0200_0FFE, 8191),
            (0x0800_0000, 16383),
            (0x1FFF_FFFD, 32767),
            (0x6B8B_4567, 60070),
            (0x79AB_CDEF, 63894),
            (0x7FED_CBA9, 65517),
            (0x7FFF_FFFF, 65535),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::arrange_coins(n), expected);
        }
    }
}

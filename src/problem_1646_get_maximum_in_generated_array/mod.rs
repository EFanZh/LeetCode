pub mod iterative;

pub trait Solution {
    fn get_maximum_generated(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // See <https://oeis.org/A270362>.
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 4),
            (10, 4),
            (11, 5),
            (12, 5),
            (13, 5),
            (14, 5),
            (15, 5),
            (16, 5),
            (17, 5),
            (18, 5),
            (19, 7),
            (20, 7),
            (21, 8),
            (22, 8),
            (23, 8),
            (24, 8),
            (25, 8),
            (26, 8),
            (27, 8),
            (28, 8),
            (29, 8),
            (30, 8),
            (31, 8),
            (32, 8),
            (33, 8),
            (34, 8),
            (35, 9),
            (36, 9),
            (37, 11),
            (38, 11),
            (39, 11),
            (40, 11),
            (41, 11),
            (42, 11),
            (43, 13),
            (44, 13),
            (45, 13),
            (46, 13),
            (47, 13),
            (48, 13),
            (49, 13),
            (50, 13),
            (51, 13),
            (52, 13),
            (53, 13),
            (54, 13),
            (55, 13),
            (56, 13),
            (57, 13),
            (58, 13),
            (59, 13),
            (60, 13),
            (61, 13),
            (62, 13),
            (63, 13),
            (64, 13),
            (65, 13),
            (66, 13),
            (67, 13),
            (68, 13),
            (69, 14),
            (70, 14),
            (71, 14),
            (72, 14),
            (73, 15),
            (74, 15),
            (75, 18),
            (76, 18),
            (77, 18),
            (78, 18),
            (79, 18),
            (80, 18),
            (81, 18),
            (82, 18),
            (83, 19),
            (84, 19),
            (85, 21),
            (86, 21),
            (87, 21),
            (88, 21),
            (89, 21),
            (90, 21),
            (91, 21),
            (92, 21),
            (93, 21),
            (94, 21),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::get_maximum_generated(n), expected);
        }
    }
}

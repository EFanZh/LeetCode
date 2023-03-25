pub mod dynamic_programming;

pub trait Solution {
    fn ways(pizza: Vec<String>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["A..", "AAA", "..."] as &[_], 3), 3),
            ((&["A..", "AA.", "..."], 3), 1),
            ((&["A..", "A..", "..."], 1), 1),
            (
                (
                    &[
                        "..A.A.AAA...AAAAAA.AA..A..A.A......A.AAA.AAAAAA.AA",
                        "A.AA.A.....AA..AA.AA.A....AAA.A........AAAAA.A.AA.",
                        "A..AA.AAA..AAAAAAAA..AA...A..A...A..AAA...AAAA..AA",
                        "....A.A.AA.AA.AA...A.AA.AAA...A....AA.......A..AA.",
                        "AAA....AA.A.A.AAA...A..A....A..AAAA...A.A.A.AAAA..",
                        "....AA..A.AA..A.A...A.A..AAAA..AAAA.A.AA..AAA...AA",
                        "A..A.AA.AA.A.A.AA..A.A..A.A.AAA....AAAAA.A.AA..A.A",
                        ".AA.A...AAAAA.A..A....A...A.AAAA.AA..A.AA.AAAA.AA.",
                        "A.AA.AAAA.....AA..AAA..AAAAAAA...AA.A..A.AAAAA.A..",
                        "A.A...A.A...A..A...A.AAAA.A..A....A..AA.AAA.AA.AA.",
                        ".A.A.A....AAA..AAA...A.AA..AAAAAAA.....AA....A....",
                        "..AAAAAA..A..A...AA.A..A.AA......A.AA....A.A.AAAA.",
                        "...A.AA.AAA.AA....A..AAAA...A..AAA.AAAA.A.....AA.A",
                        "A.AAAAA..A...AAAAAAAA.AAA.....A.AAA.AA.A..A.A.A...",
                        "A.A.AA...A.A.AA...A.AA.AA....AA...AA.A..A.AA....AA",
                        "AA.A..A.AA..AAAAA...A..AAAAA.AA..AA.AA.A..AAAAA..A",
                        "...AA....AAAA.A...AA....AAAAA.A.AAAA.A.AA..AA..AAA",
                        "..AAAA..AA..A.AA.A.A.AA...A...AAAAAAA..A.AAA..AA.A",
                        "AA....AA....AA.A......AAA...A...A.AA.A.AA.A.A.AA.A",
                        "A.AAAA..AA..A..AAA.AAA.A....AAA.....A..A.AA.A.A...",
                        "..AA...AAAAA.A.A......AA...A..AAA.AA..A.A.A.AA..A.",
                        ".......AA..AA.AAA.A....A...A.AA..A.A..AAAAAAA.AA.A",
                        ".A.AAA.AA..A.A.A.A.A.AA...AAAA.A.A.AA..A...A.AAA..",
                        "A..AAAAA.A..A..A.A..AA..A...AAA.AA.A.A.AAA..A.AA..",
                        "A.AAA.A.AAAAA....AA..A.AAA.A..AA...AA..A.A.A.AA.AA",
                        ".A..AAAA.A.A.A.A.......AAAA.AA...AA..AAA..A...A.AA",
                        "A.A.A.A..A...AA..A.AAA..AAAAA.AA.A.A.A..AA.A.A....",
                        "A..A..A.A.AA.A....A...A......A.AA.AAA..A.AA...AA..",
                        ".....A..A...A.A...A..A.AA.A...AA..AAA...AA..A.AAA.",
                        "A...AA..A..AA.A.A.AAA..AA..AAA...AAA..AAA.AAAAA...",
                        "AA...AAA.AAA...AAAA..A...A..A...AA...A..AA.A...A..",
                        "A.AA..AAAA.AA.AAA.A.AA.A..AAAAA.A...A.A...A.AA....",
                        "A.......AA....AA..AAA.AAAAAAA.A.AA..A.A.AA....AA..",
                        ".A.A...AA..AA...AA.AAAA.....A..A..A.AA.A.AA...A.AA",
                        "..AA.AA.AA..A...AA.AA.AAAAAA.....A.AA..AA......A..",
                        "AAA..AA...A....A....AA.AA.AA.A.A.A..AA.AA..AAA.AAA",
                        "..AAA.AAA.A.AA.....AAA.A.AA.AAAAA..AA..AA.........",
                        ".AA..A......A.A.AAA.AAAA...A.AAAA...AAA.AAAA.....A",
                        "AAAAAAA.AA..A....AAAA.A..AA.A....AA.A...A.A....A..",
                        ".A.A.AA..A.AA.....A.A...A.A..A...AAA..A..AA..A.AAA",
                        "AAAA....A...A.AA..AAA..A.AAA..AA.........AA.AAA.A.",
                        "......AAAA..A.AAA.A..AAA...AAAAA...A.AA..A.A.AA.A.",
                        "AA......A.AAAAAAAA..A.AAA...A.A....A.AAA.AA.A.AAA.",
                        ".A.A....A.AAA..A..AA........A.AAAA.AAA.AA....A..AA",
                        ".AA.A...AA.AAA.A....A.A...A........A.AAA......A...",
                        "..AAA....A.A...A.AA..AAA.AAAAA....AAAAA..AA.AAAA..",
                        "..A.AAA.AA..A.AA.A...A.AA....AAA.A.....AAA...A...A",
                        ".AA.AA...A....A.AA.A..A..AAA.A.A.AA.......A.A...A.",
                        "...A...A.AA.A..AAAAA...AA..A.A..AAA.AA...AA...A.A.",
                        "..AAA..A.A..A..A..AA..AA...A..AA.AAAAA.A....A..A.A",
                    ],
                    8,
                ),
                641_829_390,
            ),
        ];

        for ((pizza, k), expected) in test_cases {
            assert_eq!(
                S::ways(pizza.iter().copied().map(str::to_string).collect(), k),
                expected,
            );
        }
    }
}

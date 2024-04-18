pub mod greedy_and_dynamic_programming;

pub trait Solution {
    fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[1, 2, 3, 4, 5, 6] as &[_]), 4),
            ((4, &[1, 3, 2, 5, 2, 2, 1, 6]), 4),
            (
                (
                    8,
                    &[
                        244_197_059,
                        419_273_145,
                        329_407_130,
                        44_079_526,
                        351_372_795,
                        200_588_773,
                        340_091_770,
                        851_189_293,
                        909_604_028,
                        621_703_634,
                        959_388_577,
                        989_293_607,
                        325_139_045,
                        263_977_422,
                        358_987_768,
                        108_391_681,
                        584_357_588,
                        656_476_891,
                        621_680_874,
                        867_119_215,
                        639_909_909,
                        98_831_415,
                        263_171_984,
                        236_390_093,
                        21_876_446,
                    ],
                ),
                13,
            ),
        ];

        for ((batch_size, groups), expected) in test_cases {
            assert_eq!(S::max_happy_groups(batch_size, groups.to_vec()), expected);
        }
    }
}

pub mod bounded_memoized_dynamic_programming;

pub trait Solution {
    fn soup_servings(n: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 0.625),
            (2, 0.625),
            (3, 0.625),
            (4, 0.625),
            (5, 0.625),
            (6, 0.625),
            (7, 0.625),
            (8, 0.625),
            (10, 0.625),
            (11, 0.625),
            (13, 0.625),
            (15, 0.625),
            (18, 0.625),
            (21, 0.625),
            (24, 0.625),
            (28, 0.625),
            (33, 0.625),
            (38, 0.625),
            (45, 0.625),
            (50, 0.625),
            (52, 0.65625),
            (61, 0.65625),
            (71, 0.65625),
            (82, 0.71875),
            (96, 0.71875),
            (100, 0.71875),
            (111, 0.742_187_5),
            (130, 0.757_812_5),
            (151, 0.785_156_25),
            (176, 0.796_875),
            (205, 0.817_871_093_75),
            (238, 0.827_636_718_75),
            (277, 0.852_172_851_562_5),
            (323, 0.866_699_218_75),
            (376, 0.889_633_178_710_937_5),
            (437, 0.904_058_456_420_898_4),
            (509, 0.924_045_443_534_851_1),
            (593, 0.935_930_430_889_129_6),
            (690, 0.950_555_436_313_152_3),
            (803, 0.964_968_222_047_900_8),
            (935, 0.973_525_374_143_719_1),
            (1089, 0.981_595_036_132_972_6),
            (1268, 0.988_204_449_866_558_3),
            (1476, 0.992_831_902_473_801_8),
            (1718, 0.995_849_581_274_638_1),
            (2000, 0.997_716_316_324_876_3),
            (2328, 0.998_959_296_821_838_9),
            (2711, 0.999_559_629_853_869_9),
            (3156, 0.999_836_220_003_287_2),
            (3674, 0.999_944_905_488_703_1),
            (4277, 0.999_985_341_926_333_9),
            (4450, 0.999_989_386_677_225_4),
            (4451, 0.999_990_211_307_254),
            (4475, 0.999_990_211_307_254),
            (4979, 0.999_996_738_659_996_3),
            (5797, 0.999_999_408_486_444_3),
            (6749, 0.999_999_921_259_210_4),
            (7857, 0.999_999_992_881_269_3),
            (9146, 0.999_999_999_499_838_5),
            (10648, 0.999_999_999_978_446_5),
            (12396, 0.999_999_999_999_443_7),
            (14432, 0.999_999_999_999_992_2),
            (16800, 0.999_999_999_999_999_9),
            (16801, 1.0),
            (660_295_675, 1.0),
        ];

        for (n, expected) in test_cases {
            approx::assert_ulps_eq!(S::soup_servings(n), expected);
        }
    }
}

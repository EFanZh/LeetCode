pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const DATA: [[u64; 30]; 8] = [
    [
        1,
        4,
        9,
        16,
        25,
        58,
        157,
        470,
        1_055,
        1_772,
        9_219,
        18_228,
        33_579,
        65_802,
        105_795,
        159_030,
        212_865,
        286_602,
        872_187,
        2_630_758,
        4_565_149,
        6_544_940,
        9_674_153,
        14_745_858,
        20_005_383,
        25_846_868,
        39_347_399,
        759_196_316,
        1_669_569_335,
        2_609_044_274,
    ],
    [
        1,
        3,
        7,
        15,
        136,
        287,
        499,
        741,
        1_225,
        1_881,
        2_638,
        31_730,
        80_614,
        155_261,
        230_718,
        306_985,
        399_914,
        493_653,
        1_342_501,
        2_863_752,
        5_849_644,
        9_871_848,
        14_090_972,
        18_342_496,
        22_630_320,
        28_367_695,
        36_243_482,
        44_192_979,
        71_904_751,
        155_059_889,
    ],
    [
        1,
        3,
        6,
        11,
        66,
        439,
        832,
        1_498,
        2_285,
        3_224,
        11_221,
        64_456,
        119_711,
        175_366,
        233_041,
        739_646,
        2_540_727,
        4_755_849,
        8_582_132,
        12_448_815,
        17_500_320,
        22_726_545,
        27_986_070,
        33_283_995,
        38_898_160,
        44_577_925,
        98_400_760,
        721_411_086,
        1_676_067_545,
        53_393_239_260,
    ],
    [
        1,
        3,
        6,
        10,
        16,
        104,
        356,
        638,
        1_264,
        1_940,
        3_161,
        18_912,
        37_793,
        10_125_794,
        20_526_195,
        48_237_967,
        78_560_270,
        126_193_944,
        192_171_900,
        1_000_828_708,
        1_832_161_846,
        2_664_029_984,
        3_500_161_622,
        4_336_343_260,
        6_849_225_412,
        9_446_112_364,
        12_339_666_346,
        19_101_218_022,
        31_215_959_143,
        43_401_017_264,
    ],
    [
        1, 3, 6, 10, 15, 22, 77, 188, 329, 520, 863, 1_297, 2_074, 2_942, 4_383, 12_050, 19_827, 41_849, 81_742,
        156_389, 325_250, 1_134_058, 2_043_967, 3_911_648, 7_009_551, 11_241_875, 15_507_499, 19_806_423, 24_322_577,
        28_888_231,
    ],
    [
        1,
        3,
        6,
        10,
        15,
        21,
        29,
        150,
        321,
        563,
        855,
        17_416,
        83_072,
        2_220_384,
        6_822_448,
        13_420_404,
        20_379_000,
        29_849_749,
        91_104_965,
        321_578_997,
        788_407_661,
        1_273_902_245,
        1_912_731_081,
        2_570_225_837,
        3_428_700_695,
        29_128_200_347,
        69_258_903_451,
        115_121_130_305,
        176_576_075_721,
        241_030_621_167,
    ],
    [
        1, 3, 6, 10, 15, 21, 28, 37, 158, 450, 783, 1_156, 1_570, 2_155, 5_818, 14_596, 27_727, 41_058, 67_520, 94_182,
        124_285, 154_588, 362_290, 991_116, 1_651_182, 3_148_123, 5_083_514, 7_054_305, 11_253_219, 66_619_574,
    ],
    [
        1, 3, 6, 10, 15, 21, 28, 36, 227, 509, 882, 1_346, 1_901, 2_547, 3_203, 10_089, 35_841, 63_313, 105_637,
        156_242, 782_868, 2_323_319, 4_036_490, 5_757_761, 7_586_042, 9_463_823, 11_349_704, 13_750_746, 16_185_088,
        18_627_530,
    ],
];

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        DATA[k as u32 as usize - 2][n as u32 as usize - 1] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_mirror(k: i32, n: i32) -> i64 {
        Self::k_mirror(k, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
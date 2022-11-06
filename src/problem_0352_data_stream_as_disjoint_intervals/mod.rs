pub mod ordered_map;
pub mod union_find;

pub trait SummaryRanges {
    fn new() -> Self;
    fn add_num(&mut self, val: i32);
    fn get_intervals(&self) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::SummaryRanges;

    #[allow(variant_size_differences)]
    enum Operation {
        AddNum(i32),
        GetIntervals(&'static [[i32; 2]]),
    }

    #[allow(clippy::too_many_lines)]
    pub fn run<S: SummaryRanges>() {
        use Operation::{AddNum, GetIntervals};

        let test_cases = [
            &[
                AddNum(1),
                GetIntervals(&[[1, 1]]),
                AddNum(3),
                GetIntervals(&[[1, 1], [3, 3]]),
                AddNum(7),
                GetIntervals(&[[1, 1], [3, 3], [7, 7]]),
                AddNum(2),
                GetIntervals(&[[1, 3], [7, 7]]),
                AddNum(6),
                GetIntervals(&[[1, 3], [6, 7]]),
            ] as &[_],
            &[
                AddNum(6),
                GetIntervals(&[[6, 6]]),
                AddNum(6),
                GetIntervals(&[[6, 6]]),
                AddNum(0),
                GetIntervals(&[[0, 0], [6, 6]]),
                AddNum(4),
                GetIntervals(&[[0, 0], [4, 4], [6, 6]]),
                AddNum(8),
                GetIntervals(&[[0, 0], [4, 4], [6, 6], [8, 8]]),
                AddNum(7),
                GetIntervals(&[[0, 0], [4, 4], [6, 8]]),
                AddNum(6),
                GetIntervals(&[[0, 0], [4, 4], [6, 8]]),
                AddNum(4),
                GetIntervals(&[[0, 0], [4, 4], [6, 8]]),
                AddNum(7),
                GetIntervals(&[[0, 0], [4, 4], [6, 8]]),
                AddNum(5),
                GetIntervals(&[[0, 0], [4, 8]]),
            ],
            &[
                AddNum(1),
                GetIntervals(&[[1, 1]]),
                AddNum(9),
                GetIntervals(&[[1, 1], [9, 9]]),
                AddNum(2),
                GetIntervals(&[[1, 2], [9, 9]]),
            ],
            &[
                AddNum(49),
                GetIntervals(&[[49, 49]]),
                AddNum(97),
                GetIntervals(&[[49, 49], [97, 97]]),
                AddNum(53),
                GetIntervals(&[[49, 49], [53, 53], [97, 97]]),
                AddNum(5),
                GetIntervals(&[[5, 5], [49, 49], [53, 53], [97, 97]]),
                AddNum(33),
                GetIntervals(&[[5, 5], [33, 33], [49, 49], [53, 53], [97, 97]]),
                AddNum(65),
                GetIntervals(&[[5, 5], [33, 33], [49, 49], [53, 53], [65, 65], [97, 97]]),
                AddNum(62),
                GetIntervals(&[[5, 5], [33, 33], [49, 49], [53, 53], [62, 62], [65, 65], [97, 97]]),
                AddNum(51),
                GetIntervals(&[
                    [5, 5],
                    [33, 33],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [62, 62],
                    [65, 65],
                    [97, 97],
                ]),
                AddNum(100),
                GetIntervals(&[
                    [5, 5],
                    [33, 33],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [62, 62],
                    [65, 65],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(38),
                GetIntervals(&[
                    [5, 5],
                    [33, 33],
                    [38, 38],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [62, 62],
                    [65, 65],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(61),
                GetIntervals(&[
                    [5, 5],
                    [33, 33],
                    [38, 38],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [65, 65],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(45),
                GetIntervals(&[
                    [5, 5],
                    [33, 33],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [65, 65],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(74),
                GetIntervals(&[
                    [5, 5],
                    [33, 33],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [65, 65],
                    [74, 74],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(27),
                GetIntervals(&[
                    [5, 5],
                    [27, 27],
                    [33, 33],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [65, 65],
                    [74, 74],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(64),
                GetIntervals(&[
                    [5, 5],
                    [27, 27],
                    [33, 33],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(17),
                GetIntervals(&[
                    [5, 5],
                    [17, 17],
                    [27, 27],
                    [33, 33],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(36),
                GetIntervals(&[
                    [5, 5],
                    [17, 17],
                    [27, 27],
                    [33, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(17),
                GetIntervals(&[
                    [5, 5],
                    [17, 17],
                    [27, 27],
                    [33, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [97, 97],
                    [100, 100],
                ]),
                AddNum(96),
                GetIntervals(&[
                    [5, 5],
                    [17, 17],
                    [27, 27],
                    [33, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(12),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 17],
                    [27, 27],
                    [33, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(79),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 17],
                    [27, 27],
                    [33, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [79, 79],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(32),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 17],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [74, 74],
                    [79, 79],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(68),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 17],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [79, 79],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(90),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 17],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [79, 79],
                    [90, 90],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(77),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 17],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [90, 90],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(18),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 18],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 38],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [90, 90],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(39),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 18],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 39],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [90, 90],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(12),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 18],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 39],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [90, 90],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(93),
                GetIntervals(&[
                    [5, 5],
                    [12, 12],
                    [17, 18],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 39],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [90, 90],
                    [93, 93],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(9),
                GetIntervals(&[
                    [5, 5],
                    [9, 9],
                    [12, 12],
                    [17, 18],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 39],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [90, 90],
                    [93, 93],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(87),
                GetIntervals(&[
                    [5, 5],
                    [9, 9],
                    [12, 12],
                    [17, 18],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 39],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [87, 87],
                    [90, 90],
                    [93, 93],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(42),
                GetIntervals(&[
                    [5, 5],
                    [9, 9],
                    [12, 12],
                    [17, 18],
                    [27, 27],
                    [32, 33],
                    [36, 36],
                    [38, 39],
                    [42, 42],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [61, 62],
                    [64, 65],
                    [68, 68],
                    [74, 74],
                    [77, 77],
                    [79, 79],
                    [87, 87],
                    [90, 90],
                    [93, 93],
                    [96, 97],
                    [100, 100],
                ]),
                AddNum(60),
                AddNum(71),
                AddNum(12),
                AddNum(45),
                AddNum(55),
                AddNum(40),
                AddNum(78),
                AddNum(81),
                AddNum(26),
                AddNum(70),
                AddNum(61),
                AddNum(56),
                AddNum(66),
                AddNum(33),
                AddNum(7),
                AddNum(70),
                AddNum(1),
                AddNum(11),
                AddNum(92),
                AddNum(51),
                AddNum(90),
                AddNum(100),
                AddNum(85),
                AddNum(80),
                GetIntervals(&[
                    [1, 1],
                    [5, 5],
                    [7, 7],
                    [9, 9],
                    [11, 12],
                    [17, 18],
                    [26, 27],
                    [32, 33],
                    [36, 36],
                    [38, 40],
                    [42, 42],
                    [45, 45],
                    [49, 49],
                    [51, 51],
                    [53, 53],
                    [55, 56],
                    [60, 62],
                    [64, 66],
                    [68, 68],
                    [70, 71],
                    [74, 74],
                    [77, 81],
                    [85, 85],
                    [87, 87],
                    [90, 90],
                    [92, 93],
                    [96, 97],
                    [100, 100],
                ]),
            ],
        ];

        for operations in test_cases {
            let mut summary_ranges = S::new();

            for operation in operations {
                match *operation {
                    AddNum(val) => summary_ranges.add_num(val),
                    GetIntervals(expected) => assert_eq!(summary_ranges.get_intervals(), expected),
                }
            }
        }
    }
}

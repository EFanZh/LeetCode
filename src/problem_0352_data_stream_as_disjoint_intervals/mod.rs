pub mod union_find;

pub trait SummaryRanges {
    fn new() -> Self;
    fn add_num(&mut self, val: i32);
    fn get_intervals(&self) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::SummaryRanges;

    #[derive(Clone, Copy)]
    enum Operation<'a> {
        AddNum(i32),
        GetIntervals(&'a [[i32; 2]]),
    }

    pub fn run<S: SummaryRanges>() {
        use Operation::{AddNum, GetIntervals};

        let test_cases = [&[
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
        ] as &[_]];

        for operations in test_cases.iter().copied() {
            let mut summary_ranges = S::new();

            for &operation in operations {
                match operation {
                    AddNum(val) => summary_ranges.add_num(val),
                    GetIntervals(expected) => assert_eq!(summary_ranges.get_intervals(), expected),
                }
            }
        }
    }
}

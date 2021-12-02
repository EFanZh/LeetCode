pub mod two_heaps;

pub trait MedianFinder {
    fn new() -> Self;
    fn add_num(&mut self, num: i32);
    fn find_median(&self) -> f64;
}

#[cfg(test)]
mod tests {
    use super::MedianFinder;
    use std::f64;

    enum Operation {
        AddNum(i32),
        FindMedian(f64),
    }

    #[allow(clippy::manual_assert)]
    pub fn run<M: MedianFinder>() {
        use Operation::{AddNum, FindMedian};

        let test_cases = [
            &[AddNum(1), AddNum(2), FindMedian(1.5), AddNum(3), FindMedian(2.0)] as &[_],
            &[
                AddNum(-1),
                FindMedian(-1.0),
                AddNum(-2),
                FindMedian(-1.5),
                AddNum(-3),
                FindMedian(-2.0),
                AddNum(-4),
                FindMedian(-2.5),
                AddNum(-5),
                FindMedian(-3.0),
            ],
        ];

        for operations in test_cases {
            let mut median_finder = M::new();

            for operation in operations {
                match *operation {
                    AddNum(num) => median_finder.add_num(num),
                    FindMedian(expected) => approx::assert_ulps_eq!(median_finder.find_median(), expected),
                }
            }
        }
    }
}

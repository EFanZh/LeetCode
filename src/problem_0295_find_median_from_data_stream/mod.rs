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

    pub fn run<M: MedianFinder>() {
        use Operation::{AddNum, FindMedian};

        let test_cases = [&[AddNum(1), AddNum(2), FindMedian(1.5), AddNum(3), FindMedian(2.0)] as &[_]];

        for operations in test_cases.iter().copied() {
            let mut median_finder = M::new();

            for operation in operations {
                match *operation {
                    AddNum(num) => median_finder.add_num(num),
                    FindMedian(expected) => assert!((median_finder.find_median() - expected).abs() < f64::EPSILON),
                }
            }
        }
    }
}

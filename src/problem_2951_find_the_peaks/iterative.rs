pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        (1..mountain.len().checked_sub(1).unwrap())
            .filter_map(|i| (mountain[i] > mountain[i - 1].max(mountain[i + 1])).then_some(i as _))
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        Self::find_peaks(mountain)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut xs = points.into_iter().map(|point| point[0] as u32).collect::<Box<_>>();

        xs.sort_unstable();

        let mut iter = xs.iter().copied();
        let mut prev: u32 = iter.next().unwrap();
        let mut result = 0;

        for x in iter {
            result = result.max(x - prev);
            prev = x;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        Self::max_width_of_vertical_area(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

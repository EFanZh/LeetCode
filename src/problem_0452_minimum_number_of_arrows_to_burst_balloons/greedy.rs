pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;

        points.sort_unstable_by_key(|point| point[1]);

        points.split_first().map_or(0, |(first, rest)| {
            let mut result = 1;
            let mut prev = first[1];

            for point in rest {
                let [start, end] = point.as_slice().try_into().unwrap();

                if start > prev {
                    result += 1;
                    prev = end;
                }
            }

            result
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        Self::find_min_arrow_shots(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut points = points.into_iter().map(|point| point[0]).collect::<Vec<_>>();

        points.sort_unstable();

        let mut result = 0;
        let mut covered = i32::MIN;

        for &point in &points {
            if point > covered {
                result += 1;
                covered = point + w;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        Self::min_rectangles_to_cover_points(points, w)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

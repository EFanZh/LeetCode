pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn calculate_triangle_area_times_2(p0: (i32, i32), p1: (i32, i32), p2: (i32, i32)) -> i32 {
        (p0.0 * (p1.1 - p2.1) + p1.0 * (p2.1 - p0.1) + p2.0 * (p0.1 - p1.1)).abs()
    }

    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let points = points
            .iter()
            .map(|p| {
                let [x, y]: [i32; 2] = p.as_slice().try_into().unwrap();

                (x, y)
            })
            .collect::<Vec<_>>();

        let mut max_area_times_2 = 0;

        for (first_p1, &p0) in (1..).zip(&points) {
            for (first_p2, &p1) in (first_p1 + 1..).zip(&points[first_p1..]) {
                for &p2 in &points[first_p2..] {
                    max_area_times_2 = max_area_times_2.max(Self::calculate_triangle_area_times_2(p0, p1, p2));
                }
            }
        }

        f64::from(max_area_times_2) * 0.5
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        Self::largest_triangle_area(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

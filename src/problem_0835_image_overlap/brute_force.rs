pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter::Zip;
use std::slice::Iter;

impl Solution {
    fn get_overlaps<'a>(overlap: impl Iterator<Item = impl Iterator<Item = (&'a i32, &'a i32)>>) -> usize {
        overlap
            .map(|row| row.map(|(lhs, rhs)| lhs & rhs).sum::<i32>())
            .sum::<i32>() as _
    }

    fn zip_left(start: usize) -> impl for<'a> Fn((&'a Vec<i32>, &'a Vec<i32>)) -> Zip<Iter<'a, i32>, Iter<'a, i32>> {
        move |(row_1, row_2)| row_1.iter().zip(&row_2[start..])
    }

    fn zip_right(start: usize) -> impl for<'a> Fn((&'a Vec<i32>, &'a Vec<i32>)) -> Zip<Iter<'a, i32>, Iter<'a, i32>> {
        move |(row_1, row_2)| row_1[start..].iter().zip(row_2)
    }

    fn zip_row<'a>((row_1, row_2): (&'a Vec<i32>, &'a Vec<i32>)) -> Zip<Iter<'a, i32>, Iter<'a, i32>> {
        row_1.iter().zip(row_2)
    }

    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut result = 0;

        let zip_all = || img1.iter().zip(&img2);
        let zip_top = |start| img1.iter().zip(&img2[start..]);
        let zip_bottom = |start| img1[start..].iter().zip(&img2);

        // Full.

        result = result.max(Self::get_overlaps(zip_all().map(Self::zip_row)));

        for start in 1..n {
            let max_area = (n - start) * n;

            // Left.

            if max_area > result {
                result = result.max(Self::get_overlaps(zip_all().map(Self::zip_left(start))));
            } else {
                break;
            }

            // Right.

            if max_area > result {
                result = result.max(Self::get_overlaps(zip_all().map(Self::zip_right(start))));
            } else {
                break;
            }
        }

        for y_start in 1..n {
            let max_area = n * (n - y_start);

            // Top.

            if max_area > result {
                result = result.max(Self::get_overlaps(zip_top(y_start).map(Self::zip_row)));
            } else {
                break;
            }

            // Bottom.

            if max_area > result {
                result = result.max(Self::get_overlaps(zip_bottom(y_start).map(Self::zip_row)));
            } else {
                break;
            }

            for x_start in 1..n {
                let max_area = (n - x_start) * (n - y_start);

                // Top left.

                if max_area > result {
                    result = result.max(Self::get_overlaps(zip_top(y_start).map(Self::zip_left(x_start))));
                } else {
                    break;
                }

                // Top right.

                if max_area > result {
                    result = result.max(Self::get_overlaps(zip_top(y_start).map(Self::zip_right(x_start))));
                } else {
                    break;
                }

                // Bottom left.

                if max_area > result {
                    result = result.max(Self::get_overlaps(zip_bottom(y_start).map(Self::zip_left(x_start))));
                } else {
                    break;
                }

                // Bottom right.

                if max_area > result {
                    result = result.max(Self::get_overlaps(zip_bottom(y_start).map(Self::zip_right(x_start))));
                } else {
                    break;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        Self::largest_overlap(img1, img2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

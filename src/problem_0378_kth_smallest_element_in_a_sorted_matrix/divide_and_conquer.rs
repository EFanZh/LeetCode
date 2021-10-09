pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::ops::Index;

#[derive(Clone)]
struct MatrixRef<'a> {
    matrix: &'a [Vec<i32>],
    start: usize,
    end: usize,
}

impl MatrixRef<'_> {
    fn len(&self) -> usize {
        self.end - self.start
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = &[i32]> + ExactSizeIterator + '_ {
        let range = self.start..self.end;

        self.matrix[range.clone()].iter().map(move |row| &row[range.clone()])
    }
}

impl Index<usize> for MatrixRef<'_> {
    type Output = [i32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[self.start + index][self.start..self.end]
    }
}

// http://www.cse.yorku.ca/~andy/pubs/X+Y.pdf.

impl Solution {
    fn count_with(matrix: MatrixRef, depth: u32, mut pred: impl FnMut(i32) -> bool) -> usize {
        let step = 1 << depth;
        let n = ((matrix.len() - 1) >> depth) + 1;
        let mut result = 0;
        let mut colum_index = 0;

        for (i, row) in matrix.iter().step_by(step).enumerate().rev() {
            loop {
                if let Some(&x) = row.get(colum_index) {
                    if pred(x) {
                        colum_index += step;
                    } else {
                        result += colum_index >> depth;

                        break;
                    }
                } else {
                    return result + n * (i + 1);
                }
            }
        }

        result
    }

    fn count_less_than(matrix: MatrixRef, depth: u32, value: i32) -> usize {
        Self::count_with(matrix, depth, |x| x < value)
    }

    fn count_less_than_or_equal_to(matrix: MatrixRef, depth: u32, value: i32) -> usize {
        Self::count_with(matrix, depth, |x| x <= value)
    }

    fn select_between(matrix: MatrixRef, depth: u32, low: i32, high: i32, target: &mut Vec<i32>) {
        let step = 1 << depth;
        let mut min_column_index = 0;

        for row in matrix.iter().step_by(step).rev() {
            let mut x = loop {
                if let Some(&x) = row.get(min_column_index) {
                    if x <= low {
                        min_column_index += step;
                    } else {
                        break x;
                    }
                } else {
                    return;
                }
            };

            // `x` is the first value that is greater than `low`;

            let mut column_index = min_column_index;

            while x < high {
                target.push(x);
                column_index += step;

                if let Some(&next_x) = row.get(column_index) {
                    x = next_x;
                } else {
                    break;
                }
            }
        }
    }

    fn select_pivot(length: usize, rng: &mut impl Hasher) -> usize {
        rng.write_usize(length);

        (rng.finish() % (length as u64)) as _
    }

    fn partition(values: &mut [i32], rng: &mut impl Hasher) -> usize {
        let length = values.len();
        let pivot = Self::select_pivot(length, rng);

        values.swap(pivot, length - 1);

        let mut left = 0;
        let mut right = length - 1;
        let key = values[right];

        'outer: while left != right {
            if values[left] < key {
                left += 1;
            } else {
                loop {
                    right -= 1;

                    if left == right {
                        break 'outer;
                    }

                    if values[right] < key {
                        values.swap(left, right);
                        left += 1;

                        break;
                    }
                }
            }
        }

        values.swap(left, length - 1);

        left
    }

    fn select_nth(mut values: &mut [i32], mut n: usize, rng: &mut impl Hasher) -> i32 {
        while values.len() > 1 {
            let pivot = Self::partition(values, rng);

            match pivot.cmp(&n) {
                Ordering::Less => {
                    values = &mut values[pivot + 1..];
                    n -= pivot + 1;
                }
                Ordering::Equal => break,
                Ordering::Greater => values = &mut values[..pivot],
            }
        }

        values[n]
    }

    fn bi_select(
        matrix: MatrixRef,
        depth: u32,
        low_rank: usize,
        high_rank: usize,
        buffer: &mut Vec<i32>,
        rng: &mut impl Hasher,
    ) -> (i32, i32) {
        let length = ((matrix.len() - 1) >> depth) + 1;
        let step = 1 << depth;

        if length == 2 {
            let first_row = &matrix[0];
            let second_row = &matrix[step];
            let a = first_row[0];
            let b = first_row[step];
            let c = second_row[0];
            let d = second_row[step];
            let sorted = if c < b { [a, c, b, d] } else { [a, b, c, d] };

            (sorted[low_rank], sorted.get(high_rank).copied().unwrap_or(i32::MAX))
        } else {
            let effective_length = if length % 2 == 0 { length - 1 } else { length };
            let next_low_rank = low_rank / 4;
            let next_high_rank = (high_rank + effective_length * 2 + 1) / 4;
            let (low, high) = Self::bi_select(matrix.clone(), depth + 1, next_low_rank, next_high_rank, buffer, rng);
            let less_than_high_value = Self::count_less_than(matrix.clone(), depth, high);
            let less_than_or_equal_to_low_value = Self::count_less_than_or_equal_to(matrix.clone(), depth, low);

            if less_than_or_equal_to_low_value <= low_rank {
                if less_than_high_value <= low_rank {
                    (high, high)
                } else {
                    Self::select_between(matrix, depth, low, high, buffer);

                    let result_low = Self::select_nth(buffer, low_rank - less_than_or_equal_to_low_value, rng);

                    let result_high = if less_than_high_value <= high_rank {
                        high
                    } else {
                        Self::select_nth(buffer, high_rank - less_than_or_equal_to_low_value, rng)
                    };

                    buffer.clear();

                    (result_low, result_high)
                }
            } else {
                let result_high = if less_than_or_equal_to_low_value <= high_rank {
                    if less_than_high_value <= high_rank {
                        high
                    } else {
                        Self::select_between(matrix, depth, low, high, buffer);

                        let result_high = Self::select_nth(buffer, high_rank - less_than_or_equal_to_low_value, rng);

                        buffer.clear();

                        result_high
                    }
                } else {
                    low
                };

                (low, result_high)
            }
        }
    }

    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let k = k as usize;

        let (start, end, k) = if k < n {
            (0, k, k)
        } else if k < n.pow(2) - (n - 1) {
            (0, n, k)
        } else {
            let start = k - (n.pow(2) - (n - 1));

            (start, n, (n - start).pow(2) - (n.pow(2) - k))
        };

        let matrix = MatrixRef {
            matrix: &matrix,
            start,
            end,
        };

        if k == 1 {
            matrix[0][0]
        } else {
            Self::bi_select(matrix, 0, k - 1, k - 1, &mut Vec::new(), &mut DefaultHasher::new()).0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::kth_smallest(matrix, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

struct VectorRef<'a> {
    data: &'a [i32],
    length: usize,
    row: usize,
}

impl VectorRef<'_> {
    fn get(&self, index: usize) -> Option<(u32, u32)> {
        (index < self.length).then(|| (self.data[index] as _, self.data[self.data.len() - 1 - self.row] as _))
    }
}

#[derive(Clone)]
struct MatrixRef<'a> {
    data: &'a [i32],
    length: usize,
}

impl MatrixRef<'_> {
    fn len(&self) -> usize {
        self.length
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = VectorRef> + ExactSizeIterator + '_ {
        (0..self.length).map(move |index| VectorRef {
            data: self.data,
            length: self.length,
            row: index,
        })
    }

    fn get(&self, index: usize) -> VectorRef {
        assert!(index < self.length);

        VectorRef {
            data: self.data,
            length: self.length,
            row: index,
        }
    }
}

// http://www.cse.yorku.ca/~andy/pubs/X+Y.pdf.

impl Solution {
    fn compare_fraction(lhs: (u32, u32), rhs: (u32, u32)) -> Ordering {
        (lhs.0 * rhs.1).cmp(&(lhs.1 * rhs.0))
    }

    fn count_with(matrix: MatrixRef, depth: u32, mut pred: impl FnMut((u32, u32)) -> bool) -> usize {
        let step = 1 << depth;
        let n = ((matrix.len() - 1) >> depth) + 1;
        let mut result = 0;
        let mut colum_index = 0;

        for (i, row) in matrix.iter().step_by(step).enumerate().rev() {
            loop {
                if let Some(x) = row.get(colum_index) {
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

    fn count_less_than(matrix: MatrixRef, depth: u32, value: (u32, u32)) -> usize {
        Self::count_with(matrix, depth, |x| Self::compare_fraction(x, value) == Ordering::Less)
    }

    fn count_less_than_or_equal_to(matrix: MatrixRef, depth: u32, value: (u32, u32)) -> usize {
        Self::count_with(matrix, depth, |x| Self::compare_fraction(x, value) != Ordering::Greater)
    }

    fn select_between(matrix: MatrixRef, depth: u32, low: (u32, u32), high: (u32, u32), target: &mut Vec<(u32, u32)>) {
        let step = 1 << depth;
        let mut min_column_index = 0;

        for row in matrix.iter().step_by(step).rev() {
            let mut x = loop {
                if let Some(x) = row.get(min_column_index) {
                    if Self::compare_fraction(x, low) == Ordering::Greater {
                        break x;
                    }

                    min_column_index += step;
                } else {
                    return;
                }
            };

            // `x` is the first value that is greater than `low`;

            let mut column_index = min_column_index;

            while Self::compare_fraction(x, high) == Ordering::Less {
                target.push(x);
                column_index += step;

                if let Some(next_x) = row.get(column_index) {
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

    fn partition(values: &mut [(u32, u32)], rng: &mut impl Hasher) -> usize {
        let length = values.len();
        let pivot = Self::select_pivot(length, rng);

        values.swap(pivot, length - 1);

        let mut left = 0;
        let mut right = length - 1;
        let key = values[right];

        'outer: while left != right {
            if Self::compare_fraction(values[left], key) == Ordering::Less {
                left += 1;
            } else {
                loop {
                    right -= 1;

                    if left == right {
                        break 'outer;
                    }

                    if Self::compare_fraction(values[right], key) == Ordering::Less {
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

    fn select_nth(mut values: &mut [(u32, u32)], mut n: usize, rng: &mut impl Hasher) -> (u32, u32) {
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
        buffer: &mut Vec<(u32, u32)>,
        rng: &mut impl Hasher,
    ) -> ((u32, u32), (u32, u32)) {
        let length = ((matrix.len() - 1) >> depth) + 1;
        let step = 1 << depth;

        if length == 2 {
            let first_row = matrix.get(0);
            let second_row = matrix.get(step);
            let a = first_row.get(0).unwrap();
            let b = first_row.get(step).unwrap();
            let c = second_row.get(0).unwrap();
            let d = second_row.get(step).unwrap();

            let sorted = if Self::compare_fraction(c, b) == Ordering::Less {
                [a, c, b, d]
            } else {
                [a, b, c, d]
            };

            (sorted[low_rank], sorted.get(high_rank).copied().unwrap_or((1, 1)))
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

    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len() - 1;
        let k = k as usize;
        let length = n.min(k);
        let matrix = MatrixRef { data: &arr, length };

        let result = if k == 1 {
            matrix.get(0).get(0).unwrap()
        } else {
            Self::bi_select(matrix, 0, k - 1, k - 1, &mut Vec::new(), &mut DefaultHasher::new()).0
        };

        vec![result.0 as _, result.1 as _]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        Self::kth_smallest_prime_fraction(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

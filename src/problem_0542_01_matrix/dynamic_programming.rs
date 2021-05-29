pub struct Solution;

use std::iter::Rev;
use std::slice::IterMut;

trait MakeIter<'a> {
    type Iter: Iterator<Item = &'a mut i32>;

    fn make_iter(&self, slice: &'a mut [i32]) -> Self::Iter;
}

struct Forward;

impl<'a> MakeIter<'a> for Forward {
    type Iter = IterMut<'a, i32>;

    fn make_iter(&self, slice: &'a mut [i32]) -> Self::Iter {
        slice.iter_mut()
    }
}

struct Backward;

impl<'a> MakeIter<'a> for Backward {
    type Iter = Rev<IterMut<'a, i32>>;

    fn make_iter(&self, slice: &'a mut [i32]) -> Self::Iter {
        slice.iter_mut().rev()
    }
}

impl Solution {
    fn update<'a>(mut rows_iter: impl Iterator<Item = &'a mut [i32]>, make_iter: impl for<'b> MakeIter<'b>) {
        let mut prev_row = rows_iter.next().unwrap();
        let mut first_row_iter = make_iter.make_iter(prev_row);
        let mut left = *first_row_iter.next().unwrap();

        for value in first_row_iter {
            *value = (*value).min(left + 1);
            left = *value;
        }

        for row in rows_iter {
            let mut iter = make_iter.make_iter(prev_row).zip(make_iter.make_iter(row));

            let (&mut top, left) = iter.next().unwrap();

            *left = (*left).min(top + 1);

            let mut left = *left;

            for (&mut top, current) in iter {
                *current = (*current).min(top.min(left) + 1);
                left = *current;
            }

            prev_row = row;
        }
    }

    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut mat {
            for cell in row {
                if *cell != 0 {
                    *cell = i32::max_value() - 1;
                }
            }
        }

        Self::update(mat.iter_mut().map(Vec::as_mut_slice), Forward);
        Self::update(mat.iter_mut().map(Vec::as_mut_slice).rev(), Backward);

        mat
    }
}

impl super::Solution for Solution {
    fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::update_matrix(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

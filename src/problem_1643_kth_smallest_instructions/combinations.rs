pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn combinations(n: usize) -> Box<[u32]> {
        let mut result = vec![0; n * n].into_boxed_slice();
        let mut iter = result.chunks_exact_mut(n);
        let mut prev_row = iter.next().unwrap();

        prev_row[0] = 1;

        for row in iter {
            let mut top_left = 0;

            for (value, &mut top) in row.iter_mut().zip(prev_row) {
                *value = top_left + top;
                top_left = top;
            }

            prev_row = row;
        }

        result
    }

    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let [row, column]: [_; 2] = destination.try_into().ok().unwrap();
        let mut row = row as u32 as usize;
        let mut column = column as u32 as usize;
        let mut k = k as u32;
        let length = row + column;
        let combinations = Self::combinations(length);
        let mut result = String::with_capacity(length);

        let (c, count) = loop {
            let count = combinations[length * (row + column - 1) + row];

            if k <= count {
                result.push('H');
                column -= 1;

                if column == 0 {
                    break ('V', row);
                }
            } else {
                result.push('V');
                row -= 1;

                if row == 0 {
                    break ('H', column);
                }

                k -= count;
            }
        };

        result.extend(iter::repeat(c).take(count));

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        Self::kth_smallest_path(destination, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

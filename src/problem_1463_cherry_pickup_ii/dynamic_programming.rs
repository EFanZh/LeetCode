pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let columns = grid.first().map_or(0, Vec::len);
        let buffer_size = columns * (columns + 1);
        let mut buffer = vec![i32::MIN; buffer_size].into_boxed_slice();
        let (mut cache, mut temp) = buffer.split_at_mut(buffer_size / 2);
        let index = |i: usize, j: usize| i * (columns * 2 - i + 1) / 2 + j - i;
        let (first_row, rest_rows) = grid.split_first().unwrap();

        cache[columns - 1] = first_row.first().unwrap() + first_row.last().unwrap();

        for row in rest_rows.iter().map(Vec::as_slice) {
            let mut iter = row.iter().copied().enumerate();

            while let Some((i, i_count)) = iter.next() {
                // `j == i`.

                let mut count = i32::MIN;

                for top_i in i.saturating_sub(1)..(i + 2).min(columns) {
                    for top_j in top_i..(i + 2).min(columns) {
                        count = count.max(cache[index(top_i, top_j)]);
                    }
                }

                temp[index(i, i)] = count + i_count;

                // `j > i`.

                for (j, j_count) in iter.clone() {
                    let mut count = i32::MIN;

                    for top_i in i.saturating_sub(1)..(i + 2).min(columns) {
                        for top_j in j.saturating_sub(1).max(top_i)..(j + 2).min(columns) {
                            count = count.max(cache[index(top_i, top_j)]);
                        }
                    }

                    temp[index(i, j)] = count + i_count + j_count;
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache.iter().copied().max().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        Self::cherry_pickup(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

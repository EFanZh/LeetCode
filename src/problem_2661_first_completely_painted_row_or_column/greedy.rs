pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut value_to_index = vec![0; arr.len()].into_boxed_slice();

        (0_u32..)
            .zip(arr)
            .for_each(|(i, x)| value_to_index[x as u32 as usize - 1] = i);

        let columns = mat.first().map_or(0, Vec::len);
        let mut result = u32::MAX;

        for row in &mat {
            result = result.min(row.iter().fold(0, |max_index, &value| {
                max_index.max(value_to_index[value as u32 as usize - 1])
            }));
        }

        for column in 0..columns {
            result = result.min(mat.iter().fold(0, |max_index, row| {
                max_index.max(value_to_index[row[column] as u32 as usize - 1])
            }));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        Self::first_complete_index(arr, mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

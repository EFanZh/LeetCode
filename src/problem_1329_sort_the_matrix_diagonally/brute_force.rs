pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(mat: &mut [Vec<i32>], start_y: usize, start_x: usize, buffer: &mut Vec<i32>) {
        let mut y = start_y;
        let mut x = start_x;

        while let Some(&value) = mat.get(y).and_then(|row| row.get(x)) {
            buffer.push(value);

            x += 1;
            y += 1;
        }

        buffer.sort_unstable();

        y = start_y;
        x = start_x;

        for &value in &*buffer {
            mat[y][x] = value;

            x += 1;
            y += 1;
        }

        buffer.clear();
    }

    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let rows = mat.len();
        let columns = mat.first().map_or(0, Vec::len);
        let mut buffer = Vec::with_capacity(rows.min(columns));

        for i in 0..columns {
            Self::helper(&mut mat, 0, i, &mut buffer);
        }

        for i in 1..rows {
            Self::helper(&mut mat, i, 0, &mut buffer);
        }

        mat
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::diagonal_sort(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn combinations(n: u8, mut k: u8, base: u16, f: &mut impl FnMut(u16) -> bool) -> bool {
        if k == 0 {
            f(base)
        } else {
            k -= 1;

            for i in k..n {
                if Self::combinations(i, k, base | (1 << i), f) {
                    return true;
                }
            }

            false
        }
    }

    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let columns = matrix.first().map_or(0, Vec::len) as u8;

        let matrix = matrix
            .into_iter()
            .map(|row| row.into_iter().fold(0_u16, |bits, bit| (bits << 1) | bit as u16))
            .collect::<Box<_>>();

        let mut max = 0;

        Self::combinations(columns, num_select as _, 0, &mut |combination| {
            let count = matrix
                .iter()
                .fold(0, |count, &row| count + u8::from(row & combination == row));

            if count > max {
                max = count;

                if max == matrix.len() as u8 {
                    return true;
                }
            }

            false
        });

        i32::from(max)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        Self::maximum_rows(matrix, num_select)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

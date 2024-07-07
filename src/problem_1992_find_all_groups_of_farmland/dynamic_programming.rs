pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const IS_BOUNDARY: i32 = 1 << 15;
        const COLUMN_MASK: i32 = IS_BOUNDARY - 1;

        let mut land = land;
        let rows = land.len();
        let mut result = Vec::new();
        let mut row_iter = land.iter_mut();

        // First row.

        let mut prev_row = row_iter.next().unwrap();
        let mut iter = prev_row.iter_mut();
        let mut prev = iter.next().unwrap();

        *prev -= 1;

        for (x, value) in (1..).zip(iter) {
            *value = if *value == 0 {
                if *prev != -1 {
                    *prev |= IS_BOUNDARY;
                }

                -1
            } else if *prev == -1 {
                x
            } else {
                *prev
            };

            prev = value;
        }

        if *prev != -1 {
            *prev |= IS_BOUNDARY;
        }

        for (y, row) in (1..).zip(row_iter) {
            let mut iter = prev_row.iter_mut().zip(&mut *row);
            let (top, mut prev) = iter.next().unwrap();

            *prev = if *prev == 0 {
                if *top != -1 && (*top & IS_BOUNDARY != 0) {
                    result.push(vec![*top >> 16, *top & COLUMN_MASK, y - 1, 0]);
                }

                -1
            } else if *top == -1 {
                y << 16
            } else {
                *top
            };

            for (x, (top, value)) in (1..).zip(iter) {
                *value = if *value == 0 {
                    if *top != -1 && (*top & IS_BOUNDARY != 0) {
                        result.push(vec![*top >> 16, *top & COLUMN_MASK, y - 1, x]);
                    }

                    if *prev != -1 {
                        *prev |= IS_BOUNDARY;
                    }

                    -1
                } else if *top == -1 {
                    if *prev == -1 {
                        (y << 16) | x
                    } else {
                        *prev
                    }
                } else {
                    *top
                };

                prev = value;
            }

            if *prev != -1 {
                *prev |= IS_BOUNDARY;
            }

            prev_row = row;
        }

        for (x, value) in (0..).zip(prev_row) {
            if *value != -1 && (*value & IS_BOUNDARY != 0) {
                result.push(vec![*value >> 16, *value & COLUMN_MASK, rows as i32 - 1, x]);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::find_farmland(land)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

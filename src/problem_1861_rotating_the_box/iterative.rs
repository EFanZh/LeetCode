pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn rotate_the_box(r#box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut r#box = r#box;

        // Process stones in place.

        for row in r#box.iter_mut().map(Vec::as_mut_slice) {
            let mut i = row.len();
            let mut bottom = i;

            loop {
                i = i.wrapping_sub(1);

                if let Some(&c) = row.get(i) {
                    match c {
                        '#' => {
                            bottom = bottom.wrapping_sub(1);

                            row[bottom] = '#';
                        }
                        '*' => {
                            row[i + 1..bottom].fill('.');

                            bottom = i;
                        }
                        _ => {}
                    }
                } else {
                    break;
                }
            }

            row[..bottom].fill('.');
        }

        // Rotate.

        let columns = r#box.first().map_or(0, Vec::len);

        (0..columns)
            .map(|i| r#box.iter().rev().map(|row| row[i]).collect())
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate_the_box(r#box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        Self::rotate_the_box(r#box)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

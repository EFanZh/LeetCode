pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut a = a;

        for row in &mut a {
            row.reverse();

            for cell in row {
                *cell = 1 - *cell;
            }
        }

        a
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::flip_and_invert_image(a)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

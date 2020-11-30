pub struct Solution;

impl Solution {
    pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut a {
            row.reverse();

            for cell in row {
                *cell = 1 - *cell;
            }
        }

        a
    }
}

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

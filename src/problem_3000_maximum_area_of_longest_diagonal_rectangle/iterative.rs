pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions
            .iter()
            .map(|dimension| {
                let [length, width] = <[_; 2]>::map(dimension.as_slice().try_into().ok().unwrap(), i32::cast_unsigned);

                (length.pow(2) + width.pow(2), length * width)
            })
            .max()
            .unwrap()
            .1
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        Self::area_of_max_diagonal(dimensions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

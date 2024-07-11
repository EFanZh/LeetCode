pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let [first, second]: &[_; 2] = edges[..2].try_into().ok().unwrap();
        let &[from_0, to_0]: &[_; 2] = first.as_slice().try_into().ok().unwrap();
        let &[from_1, to_1]: &[_; 2] = second.as_slice().try_into().ok().unwrap();

        if from_0 == from_1 || from_0 == to_1 {
            from_0
        } else {
            to_0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        Self::find_center(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

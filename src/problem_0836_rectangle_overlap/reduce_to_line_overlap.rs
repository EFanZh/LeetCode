pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn decode(rec: &[i32]) -> (i32, i32, i32, i32) {
        <[_; 4]>::try_from(rec).ok().unwrap().into()
    }

    fn overlaps(start_1: i32, end_1: i32, start_2: i32, end_2: i32) -> bool {
        if start_2 < start_1 {
            start_1 < end_2
        } else {
            start_2 < end_1
        }
    }

    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let rec1 = Self::decode(&rec1);
        let rec2 = Self::decode(&rec2);

        Self::overlaps(rec1.0, rec1.2, rec2.0, rec2.2) && Self::overlaps(rec1.1, rec1.3, rec2.1, rec2.3)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        Self::is_rectangle_overlap(rec1, rec2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

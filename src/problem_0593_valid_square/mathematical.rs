pub struct Solution;

use std::convert::TryInto;

impl Solution {
    fn displacement((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
        (x2 - x1, y2 - y1)
    }

    fn squared_norm((x, y): (i32, i32)) -> i32 {
        x * x + y * y
    }

    fn inner_product((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
        x1 * x2 + y1 * y2
    }

    fn add((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
        (x1 + x2, y1 + y2)
    }

    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let [x1, y1]: [_; 2] = p1.as_slice().try_into().unwrap();
        let [x2, y2]: [_; 2] = p2.as_slice().try_into().unwrap();
        let [x3, y3]: [_; 2] = p3.as_slice().try_into().unwrap();
        let [x4, y4]: [_; 2] = p4.as_slice().try_into().unwrap();
        let mut points = [(x1, y1), (x2, y2), (x3, y3), (x4, y4)];

        points.sort_unstable();

        let [p1, p2, p3, p4] = points;

        if p1 == p2 {
            false
        } else {
            let vec_12 = Self::displacement(p1, p2);
            let vec_24 = Self::displacement(p2, p4);

            Self::squared_norm(vec_12) == Self::squared_norm(vec_24)
                && Self::inner_product(vec_12, vec_24) == 0
                && p4 == Self::add(p3, vec_12)
        }
    }
}

impl super::Solution for Solution {
    fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        Self::valid_square(p1, p2, p3, p4)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

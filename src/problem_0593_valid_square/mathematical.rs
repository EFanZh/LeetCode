pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
        let p1 = <(_, _)>::from(<[_; 2]>::try_from(p1).ok().unwrap());
        let p2 = <(_, _)>::from(<[_; 2]>::try_from(p2).ok().unwrap());
        let p3 = <(_, _)>::from(<[_; 2]>::try_from(p3).ok().unwrap());
        let p4 = <(_, _)>::from(<[_; 2]>::try_from(p4).ok().unwrap());
        let mut points = [p1, p2, p3, p4];

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

// ------------------------------------------------------ snip ------------------------------------------------------ //

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

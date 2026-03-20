pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut east = 0;
        let mut west = 0;
        let mut north = 0;
        let mut south = 0;

        s.bytes()
            .fold(0, |max, c| {
                match c {
                    b'E' => east += 1,
                    b'W' => west += 1,
                    b'N' => north += 1,
                    _ => south += 1,
                }

                let (east, west) = if west < east { (west, east) } else { (east, west) };
                let (north, south) = if south < north { (south, north) } else { (north, south) };

                max.max((west - east) + (south - north) + u32::min(east + north, k.cast_unsigned()) * 2)
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(s: String, k: i32) -> i32 {
        Self::max_distance(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

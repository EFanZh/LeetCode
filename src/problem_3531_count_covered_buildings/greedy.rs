pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn extend_range(range: &mut (u32, u32), value: u32) {
        if range.0 == u32::MAX {
            *range = (value, value);
        } else if value < range.0 {
            range.0 = value;
        } else if value > range.1 {
            range.1 = value;
        }
    }

    fn covered(range: (u32, u32), value: u32) -> bool {
        value > range.0 && value < range.1
    }

    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let buildings = buildings
            .into_iter()
            .map(|building| {
                <(_, _)>::from(<[_; 2]>::map(building.try_into().ok().unwrap(), i32::cast_unsigned).map(|x| x - 1))
            })
            .collect::<Vec<_>>();

        let n = n.cast_unsigned() as usize;
        let mut rows = vec![(u32::MAX, u32::MAX); n].into_boxed_slice();
        let mut columns = vec![(u32::MAX, u32::MAX); n].into_boxed_slice();

        for &(x, y) in &buildings {
            Self::extend_range(&mut rows[y as usize], x);
            Self::extend_range(&mut columns[x as usize], y);
        }

        let mut result = 0;

        for &(x, y) in &buildings {
            result += i32::from(Self::covered(rows[y as usize], x) && Self::covered(columns[x as usize], y));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        Self::count_covered_buildings(n, buildings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

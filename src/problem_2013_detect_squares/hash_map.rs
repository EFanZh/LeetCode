// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct DetectSquares {
    points: HashMap<u16, HashMap<u16, u16>>,
}

impl DetectSquares {
    fn new() -> Self {
        Self { points: HashMap::new() }
    }

    fn add(&mut self, point: Vec<i32>) {
        let [x, y]: [_; 2] = point.try_into().ok().unwrap();
        let x = x as u16;
        let y = y as u16;

        match self.points.entry(x) {
            Entry::Occupied(entry) => match entry.into_mut().entry(y) {
                Entry::Occupied(entry) => *entry.into_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(HashMap::from([(y, 1)]));
            }
        }
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let [x_0, y_0]: [_; 2] = point.try_into().ok().unwrap();
        let x_0 = x_0 as u16;
        let y_0 = y_0 as u16;
        let mut result = 0;

        if let Some(column_0) = self.points.get(&x_0) {
            for (&y_1, &count_0) in column_0 {
                if y_0 == y_1 {
                    continue;
                }

                let d = y_1.wrapping_sub(y_0);

                for x_1 in [x_0.wrapping_sub(d), x_0.wrapping_add(d)] {
                    if let Some(column_1) = self.points.get(&x_1) {
                        if let Some(&count_1) = column_1.get(&y_0) {
                            if let Some(&count_2) = column_1.get(&y_1) {
                                result += u32::from(count_0) * u32::from(count_1) * u32::from(count_2);
                            }
                        }
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::DetectSquares for DetectSquares {
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, point: Vec<i32>) {
        self.add(point);
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        self.count(point)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::DetectSquares>();
    }
}

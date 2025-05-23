pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut distance_map = vec![HashMap::new(); points.len()];

        for (i, p1) in points[..points.len() - 1].iter().enumerate() {
            let [x1, y1] = p1.as_slice().try_into().unwrap();

            for (j, p2) in points.iter().enumerate().skip(i + 1) {
                let [x2, y2] = p2.as_slice().try_into().unwrap();
                let dx = x2 - x1;
                let dy = y2 - y1;
                let distance = dx * dx + dy * dy;

                distance_map[i].entry(distance).and_modify(|c| *c += 1).or_insert(1);
                distance_map[j].entry(distance).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        let mut result = 0;

        for distances in distance_map {
            for count in distances.values() {
                result += count * (count - 1);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        Self::number_of_boomerangs(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

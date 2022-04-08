pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;
use std::mem;

impl Solution {
    fn merge(skyline: &mut Vec<[i32; 2]>, left: usize, middle: usize, buffer: &mut Vec<[i32; 2]>) {
        let (skyline_1, skyline_2) = skyline[left..].split_at(middle - left);
        let mut iter_1 = skyline_1.iter().copied();
        let mut iter_2 = skyline_2.iter().copied();

        let [mut x_1, mut height_1] = iter_1.next().unwrap();
        let [mut x_2, mut height_2] = iter_2.next().unwrap();
        let mut prev_height_1 = 0;
        let mut prev_height_2 = 0;
        let mut prev_height = 0;

        loop {
            if x_1 == x_2 {
                let height = height_1.max(height_2);

                if height != prev_height {
                    prev_height = height;

                    buffer.push([x_1, height]);
                }

                match (iter_1.next(), iter_2.next()) {
                    (None, None) => {
                        skyline.splice(left.., buffer.drain(..));

                        return;
                    }
                    (None, Some([new_x_2, new_height_2])) => {
                        x_2 = new_x_2;
                        height_2 = new_height_2;

                        break;
                    }
                    (Some([new_x_1, new_height_1]), None) => {
                        x_2 = new_x_1;
                        height_2 = new_height_1;
                        mem::swap(&mut iter_1, &mut iter_2);

                        break;
                    }
                    (Some([new_x_1, new_height_1]), Some([new_x_2, new_height_2])) => {
                        prev_height_1 = mem::replace(&mut height_1, new_height_1);
                        prev_height_2 = mem::replace(&mut height_2, new_height_2);
                        x_1 = new_x_1;
                        x_2 = new_x_2;
                    }
                }
            } else {
                if x_1 > x_2 {
                    mem::swap(&mut prev_height_1, &mut prev_height_2);
                    mem::swap(&mut x_1, &mut x_2);
                    mem::swap(&mut height_1, &mut height_2);
                    mem::swap(&mut iter_1, &mut iter_2);
                }

                let height = height_1.max(prev_height_2);

                if height != prev_height {
                    prev_height = height;

                    buffer.push([x_1, height]);
                }

                prev_height_1 = height_1;

                if let Some([new_x_1, new_height_1]) = iter_1.next() {
                    x_1 = new_x_1;
                    height_1 = new_height_1;
                } else {
                    break;
                }
            }
        }

        buffer.push([x_2, height_2]);
        buffer.extend(iter_2);

        skyline.splice(left.., buffer.drain(..));
    }

    #[allow(clippy::ptr_arg)] // False positive.
    fn get_skyline_helper(buildings: &[Vec<i32>], target: &mut Vec<[i32; 2]>, merge_buffer: &mut Vec<[i32; 2]>) {
        if let [building] = buildings {
            let [left, right, height]: [i32; 3] = building.as_slice().try_into().unwrap();

            target.push([left, height]);
            target.push([right, 0]);
        } else {
            let (left_buildings, right_buildings) = buildings.split_at(buildings.len() / 2);
            let left = target.len();

            Self::get_skyline_helper(left_buildings, target, merge_buffer);

            let middle = target.len();

            Self::get_skyline_helper(right_buildings, target, merge_buffer);
            Self::merge(target, left, middle, merge_buffer);
        }
    }

    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::get_skyline_helper(&buildings, &mut result, &mut Vec::new());

        result.iter().copied().map(Vec::from).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::get_skyline(buildings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

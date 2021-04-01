pub struct Solution;

use std::convert::TryInto;
use std::mem;

impl Solution {
    #[allow(clippy::similar_names)]
    fn merge(skyline: &mut Vec<[i32; 2]>, left: usize, middle: usize, buffer: &mut Vec<[i32; 2]>) {
        let (skyline_1, skyline_2) = skyline[left..].split_at(middle - left);
        let mut iter_1 = skyline_1.iter().copied();
        let mut iter_2 = skyline_2.iter().copied();

        if let (Some([mut x_1, mut y_1]), Some([mut x_2, mut y_2])) = (iter_1.next(), iter_2.next()) {
            let mut prev_y_1 = 0;
            let mut prev_y_2 = 0;
            let mut prev_y = 0;

            loop {
                if x_1 == x_2 {
                    let y = y_1.max(y_2);

                    if y != prev_y {
                        prev_y = y;

                        buffer.push([x_1, y]);
                    }

                    match (iter_1.next(), iter_2.next()) {
                        (None, None) => {
                            skyline.splice(left.., buffer.drain(..));

                            return;
                        }
                        (None, Some([new_x_2, new_y_2])) => {
                            prev_y_1 = y_1;
                            x_2 = new_x_2;
                            y_2 = new_y_2;

                            break;
                        }
                        (Some([new_x_1, new_y_1]), None) => {
                            prev_y_1 = y_2;
                            x_2 = new_x_1;
                            y_2 = new_y_1;
                            mem::swap(&mut iter_1, &mut iter_2);

                            break;
                        }
                        (Some([new_x_1, new_y_1]), Some([new_x_2, new_y_2])) => {
                            prev_y_1 = mem::replace(&mut y_1, new_y_1);
                            prev_y_2 = mem::replace(&mut y_2, new_y_2);
                            x_1 = new_x_1;
                            x_2 = new_x_2;
                        }
                    }
                } else {
                    if x_1 > x_2 {
                        mem::swap(&mut prev_y_1, &mut prev_y_2);
                        mem::swap(&mut x_1, &mut x_2);
                        mem::swap(&mut y_1, &mut y_2);
                        mem::swap(&mut iter_1, &mut iter_2);
                    }

                    let y = y_1.max(prev_y_2);

                    if y != prev_y {
                        prev_y = y;

                        buffer.push([x_1, y]);
                    }

                    prev_y_1 = y_1;

                    if let Some([new_x_1, new_y_1]) = iter_1.next() {
                        x_1 = new_x_1;
                        y_1 = new_y_1;
                    } else {
                        break;
                    }
                }
            }

            loop {
                let y = y_2.max(prev_y_1);

                if y != prev_y {
                    prev_y = y;

                    buffer.push([x_2, y]);
                }

                if let Some([new_x_2, new_y_2]) = iter_2.next() {
                    x_2 = new_x_2;
                    y_2 = new_y_2;
                } else {
                    break;
                }
            }

            skyline.splice(left.., buffer.drain(..));
        }
    }

    fn get_skyline_helper(buildings: &[Vec<i32>], target: &mut Vec<[i32; 2]>, merge_buffer: &mut Vec<[i32; 2]>) {
        match buildings {
            [] => {}
            [building] => {
                let [left, right, height]: [i32; 3] = building.as_slice().try_into().unwrap();

                target.push([left, height]);
                target.push([right, 0]);
            }
            _ => {
                let (left_buildings, right_buildings) = buildings.split_at(buildings.len() / 2);
                let left = target.len();

                Self::get_skyline_helper(left_buildings, target, merge_buffer);

                let middle = target.len();

                Self::get_skyline_helper(right_buildings, target, merge_buffer);
                Self::merge(target, left, middle, merge_buffer)
            }
        }
    }

    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::get_skyline_helper(&buildings, &mut result, &mut Vec::new());

        result.iter().copied().map(Vec::from).collect()
    }
}

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

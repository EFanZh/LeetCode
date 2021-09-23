pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let (mut sx, mut sy, mut tx, mut ty) = (sx as u32, sy as u32, tx as u32, ty as u32);

        loop {
            match tx.cmp(&sx) {
                Ordering::Less => return false,
                Ordering::Equal => return ty.checked_sub(sy).map_or(false, |diff| diff % sx == 0),
                Ordering::Greater => {
                    let (source_x, source_y, target_x, target_y) = (sy, sx, ty % tx, tx);

                    sx = source_x;
                    sy = source_y;
                    tx = target_x;
                    ty = target_y;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        Self::reaching_points(sx, sy, tx, ty)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

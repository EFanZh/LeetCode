pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::f64::consts;

impl Solution {
    pub fn internal_angles(sides: Vec<i32>) -> Vec<f64> {
        const RAD_TO_DEG: f64 = 180.0 * consts::FRAC_1_PI;

        let mut sides = <[_; 3]>::try_from(sides).ok().unwrap().map(i32::cast_unsigned);

        sides.sort_unstable();

        let [x, y, z] = sides;
        let x_squared = x * x;
        let y_squared = y * y;
        let z_squared = z * z;
        let z_doubled = z * 2;

        if z < x + y {
            let angle_1 = (f64::from(y_squared + z_squared - x_squared) / f64::from(y * z_doubled)).acos() * RAD_TO_DEG;
            let angle_2 = (f64::from(x_squared + z_squared - y_squared) / f64::from(x * z_doubled)).acos() * RAD_TO_DEG;
            let angle_3 = 180.0 - (angle_1 + angle_2);

            vec![angle_1, angle_2, angle_3]
        } else {
            Vec::new()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn internal_angles(sides: Vec<i32>) -> Vec<f64> {
        Self::internal_angles(sides)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

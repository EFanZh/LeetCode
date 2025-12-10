pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn max_even_rows(x: u32) -> u32 {
        x.isqrt()
    }

    fn max_odd_rows(x: u32) -> u32 {
        ((x * 4 + 1).isqrt() - 1) / 2
    }

    fn get_max_rows(even_rows: u32, odd_rows: u32) -> u32 {
        if odd_rows < even_rows {
            odd_rows * 2 + 1
        } else {
            even_rows * 2
        }
    }

    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        let red = red.cast_unsigned();
        let blue = blue.cast_unsigned();
        let red_even_rows = Self::max_even_rows(red);
        let red_odd_rows = Self::max_odd_rows(red);
        let blue_even_rows = Self::max_even_rows(blue);
        let blue_odd_rows = Self::max_odd_rows(blue);

        Self::get_max_rows(red_even_rows, blue_odd_rows)
            .max(Self::get_max_rows(blue_even_rows, red_odd_rows))
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        Self::max_height_of_triangle(red, blue)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

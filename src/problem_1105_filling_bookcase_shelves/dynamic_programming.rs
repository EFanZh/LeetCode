pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let books = books
            .into_iter()
            .map(|book| {
                let [thickness, height] = book.try_into().ok().unwrap();

                (thickness as u32, height as u32)
            })
            .collect::<Vec<_>>();

        let shelf_width = shelf_width as u32;
        let mut cache = Vec::with_capacity(books.len());
        let mut result = 0;

        for &(thickness, height) in &books {
            let mut level_width = thickness;
            let mut level_height = height;
            let mut min_height = result + level_height;

            for (&bottom_height, &(thickness, height)) in cache.iter().zip(&books).rev() {
                level_width += thickness;

                if level_width <= shelf_width {
                    level_height = level_height.max(height);
                    min_height = min_height.min(bottom_height + level_height);
                } else {
                    break;
                }
            }

            cache.push(result);
            result = min_height;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        Self::min_height_shelves(books, shelf_width)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

#[derive(Default)]
pub struct ProductOfNumbers {
    data: Vec<NonZeroU32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, num: i32) {
        let num = num as u32;

        if let Some(new_product) = NonZeroU32::new(self.data.last().map_or(num, |product| product.get() * num)) {
            self.data.push(new_product);
        } else {
            self.data.clear();
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let result = if let Some(&product) = self.data.last() {
            let k = k as u32 as usize;

            if let Some(&prev) = self.data.get((self.data.len() - 1).wrapping_sub(k)) {
                product.get() / prev
            } else if k == self.data.len() {
                product.get()
            } else {
                0
            }
        } else {
            0
        };

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::ProductOfNumbers for ProductOfNumbers {
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, num: i32) {
        self.add(num);
    }

    fn get_product(&self, k: i32) -> i32 {
        self.get_product(k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::ProductOfNumbers>();
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};

pub struct MovieRentingSystem {
    prices: HashMap<(u32, u32), u32>,
    unrented: HashMap<u32, BTreeSet<(u32, u32)>>,
    rented: BTreeSet<(u32, u32, u32)>,
}

impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let _ = n;
        let mut unrented = HashMap::<u32, BTreeSet<(u32, u32)>>::new();
        let mut prices = HashMap::<(u32, u32), u32>::new();

        for entry in entries {
            let [shop, movie, price] = <[_; 3]>::map(entry.try_into().ok().unwrap(), |x| x as u32);

            match unrented.entry(movie) {
                Entry::Occupied(entry) => {
                    entry.into_mut().insert((price, shop));
                }
                Entry::Vacant(entry) => {
                    entry.insert(BTreeSet::from([(price, shop)]));
                }
            }

            prices.insert((movie, shop), price);
        }

        Self {
            unrented,
            prices,
            rented: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(state) = self.unrented.get(&(movie as _)) {
            result.extend(state.iter().take(5).map(|&(_, shop)| shop as i32));
        }

        result
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let (shop, movie) = (shop as u32, movie as u32);
        let state = self.unrented.get_mut(&movie).unwrap();
        let price = self.prices[&(movie, shop)];

        state.remove(&(price, shop));
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let (shop, movie) = (shop as u32, movie as u32);
        let state = self.unrented.get_mut(&movie).unwrap();
        let price = self.prices[&(movie, shop)];

        state.insert((price, shop));
        self.rented.remove(&(price, shop, movie));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented
            .iter()
            .take(5)
            .map(|&(_, shop, movie)| vec![shop as _, movie as _])
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MovieRentingSystem for MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        Self::new(n, entries)
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.search(movie)
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        self.rent(shop, movie);
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        self.drop(shop, movie);
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.report()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MovieRentingSystem>();
    }
}

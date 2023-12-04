extern crate fxhash;
use core::hash::Hash;
use fxhash::FxHashMap;

#[derive(Debug)]
pub struct CountingSet<T> {
    set: FxHashMap<T, usize>,
}

impl<T> CountingSet<T>
where
    T: Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self {
            set: FxHashMap::default(),
        }
    }

    pub fn increase_by_one(&mut self, item: T) {
        self.increase_with_many(item, 1);
    }

    pub fn increase_with_many(&mut self, item: T, amount: usize) {
        self.set
            .entry(item)
            .and_modify(|count| *count += amount)
            .or_insert(amount);
    }

    pub fn get(&self, item: &T) -> Option<usize> {
        self.set.get(item).copied()
    }

    pub fn reduce_by_one(&mut self, item: &T) {
        self.reduce_with_many(item, 1);
    }

    pub fn reduce_with_many(&mut self, item: &T, amount: usize) {
        if let Some(count) = self.set.get_mut(item) {
            if *count == amount {
                self.set.remove(item);
            } else if *count < amount {
                self.set.remove(item);
                if cfg!(debug_assertions) {
                    panic!("You are trying to remove a larger count than the item have. This is a potential bug in your code. If it't not a bug use `take` instead.")
                }
            } else {
                *count -= amount;
            }
            return;
        }
        if cfg!(debug_assertions) {
            panic!("You are trying to remove a item that does not exist.")
        }
    }

    pub fn take(&mut self, item: &T) -> Option<(T, usize)> {
        self.set.remove_entry(item)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl<T> Default for CountingSet<T>
where
    T: Hash + PartialEq + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hash + Eq + PartialEq> Extend<T> for CountingSet<T> {
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        iter.into_iter().for_each(|item| self.increase_by_one(item));
    }
}

impl<T, Iter: Iterator<Item = T>> From<Iter> for CountingSet<T>
where
    T: Hash + PartialEq + Eq,
{
    fn from(value: Iter) -> Self {
        let mut set = Self::default();
        set.extend(value);
        set
    }
}

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

    pub fn get_max(&self) -> Option<(&T, usize)> {
        self.iter()
            .max_by(|(_item, count), (_other, other_count)| count.cmp(other_count))
    }

    pub fn contains(&self, item: &T) -> bool {
        self.set.contains_key(item)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'_ T, usize)> + '_ {
        self.set.iter().map(|(t, count)| (t, *count))
    }

    pub fn iter_counts(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter().map(|(_, count)| count)
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
            if *count <= amount {
                self.set.remove(item);
            } else {
                *count -= amount;
            }
        }
    }

    pub fn take(&mut self, item: &T) -> Option<(T, usize)> {
        self.set.remove_entry(item)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl<T> FromIterator<T> for CountingSet<T>
where
    T: Hash + PartialEq + Eq,
{
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let mut set = Self::default();
        set.extend(iter);
        set
    }
}

impl<T> IntoIterator for CountingSet<T> {
    type Item = (T, usize);

    type IntoIter = <FxHashMap<T, usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
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
        Self::from_iter(value)
    }
}

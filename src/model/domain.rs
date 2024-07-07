use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Domain(LinkedList<i128>);

impl Domain {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&i128) -> bool,
    {
        let mut new_list = LinkedList::new();
        while let Some(value) = self.0.pop_front() {
            if f(&value) {
                new_list.push_back(value);
            }
        }
        self.0 = new_list;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromIterator<i128> for Domain {
    fn from_iter<T: IntoIterator<Item = i128>>(iter: T) -> Self {
        let collected = iter.into_iter().collect::<LinkedList<i128>>();
        Domain(collected)
    }
}

impl IntoIterator for Domain {
    type Item = i128;
    type IntoIter = std::collections::linked_list::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Domain {
    type Item = &'a i128;
    type IntoIter = std::collections::linked_list::Iter<'a, i128>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

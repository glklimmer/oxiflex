#[derive(Clone)]
pub struct Domain(Vec<i128>);

impl Domain {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&i128) -> bool,
    {
        self.0.retain(f);
    }
}

impl FromIterator<i128> for Domain {
    fn from_iter<T: IntoIterator<Item = i128>>(iter: T) -> Self {
        let collected = iter.into_iter().collect::<Vec<i128>>();
        Domain(collected)
    }
}

impl IntoIterator for Domain {
    type Item = i128;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Domain {
    type Item = &'a i128;
    type IntoIter = std::slice::Iter<'a, i128>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

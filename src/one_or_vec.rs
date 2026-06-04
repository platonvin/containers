#[derive(Debug)]
/// An optimization of Vec for a common case where its just one object.
pub enum OneOrVec<T> {
    None,
    One(T),
    Many(Vec<T>),
}

impl<T> Default for OneOrVec<T> {
    fn default() -> Self { OneOrVec::None }
}

impl<T> OneOrVec<T> {
    pub fn push(&mut self, val: T) {
        *self = match std::mem::take(self) {
            OneOrVec::None => OneOrVec::One(val),
            OneOrVec::One(existing) => OneOrVec::Many(vec![existing, val]),
            OneOrVec::Many(mut vec) => {
                vec.push(val);
                OneOrVec::Many(vec)
            }
        };
    }

    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        *self = match std::mem::take(self) {
            OneOrVec::None => OneOrVec::None,
            OneOrVec::One(val) => {
                if f(&val) {
                    OneOrVec::One(val)
                } else {
                    OneOrVec::None
                }
            }
            OneOrVec::Many(mut vec) => {
                vec.retain(f);
                match vec.len() {
                    // 0 => OneOrVec::None,
                    // 1 => OneOrVec::One(vec.pop().unwrap()),
                    // we dont shrink cause if 2 objects ever happens it might happen again
                    _ => OneOrVec::Many(vec),
                }
            }
        }
    }
}

impl<T> OneOrVec<T> {
    pub fn iter(&self) -> Iter<'_, T> { self.into_iter() }
}

pub enum Iter<'a, T> {
    None,
    One(Option<&'a T>),
    Many(std::slice::Iter<'a, T>),
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::None => None,
            Iter::One(opt) => opt.take(),
            Iter::Many(inner) => inner.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self {
            Iter::None => 0,
            Iter::One(opt) => {
                if opt.is_some() {
                    1
                } else {
                    0
                }
            }
            Iter::Many(inner) => inner.len(),
        };
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<'a, T> IntoIterator for &'a OneOrVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OneOrVec::None => Iter::None,
            OneOrVec::One(val) => Iter::One(Some(val)),
            OneOrVec::Many(vec) => Iter::Many(vec.iter()),
        }
    }
}

impl<T> OneOrVec<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.into_iter() }
}

pub enum IterMut<'a, T> {
    None,
    One(Option<&'a mut T>),
    Many(std::slice::IterMut<'a, T>),
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterMut::None => None,
            IterMut::One(opt) => opt.take(),
            IterMut::Many(inner) => inner.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self {
            IterMut::None => 0,
            IterMut::One(opt) => {
                if opt.is_some() {
                    1
                } else {
                    0
                }
            }
            IterMut::Many(inner) => inner.len(),
        };
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

impl<'a, T> IntoIterator for &'a mut OneOrVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OneOrVec::None => IterMut::None,
            OneOrVec::One(val) => IterMut::One(Some(val)),
            OneOrVec::Many(vec) => IterMut::Many(vec.iter_mut()),
        }
    }
}

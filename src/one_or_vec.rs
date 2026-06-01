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

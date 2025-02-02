use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub(crate) struct Memory(Vec<isize>);

impl Memory {
    pub fn new(data: Vec<isize>) -> Self {
        Self(data)
    }
}

impl From<String> for Memory {
    fn from(s: String) -> Self {
        let data: Vec<isize> = s.split(",").map(|s| s.parse::<isize>().unwrap()).collect();
        Self::new(data)
    }
}

impl Deref for Memory {
    type Target = Vec<isize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

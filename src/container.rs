use super::{List, Intercast};
use std::any::{TypeId, Any};
use scell::SCell;

pub struct Container {
    pub lists: Vec<SCell<List>>,
}

impl Container {
    pub fn new() -> Container {
        Container { lists: vec![] }
    }
}

impl List for Container {
    fn access(&self) -> Box<Iterator<Item=SCell<List>>> {
        Box::new(self.lists.iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn link(&mut self, other: SCell<List>) {
        self.lists.push(other);
    }

    fn acquire(&self, typeid: TypeId) -> Option<&Any> {
        if TypeId::of::<Self>() == typeid {
            Some(self)
        } else {
            None
        }
    }

    fn acquire_mut(&mut self, typeid: TypeId) -> Option<&mut Any> {
        if TypeId::of::<Self>() == typeid {
            Some(self)
        } else {
            None
        }
    }
}

struct Iter {
    /// This will always be type Container.
    container: SCell<List>,
    ix: usize,
}

impl Iterator for Iter {
    type Item = SCell<List>;

    fn next(&mut self) -> Option<Self::Item> {
        let cb = self.container.borrow();
        cb.downcast_ref::<Container>()
            .expect("quac::container::Iter::container wasn't a Container")
            .lists
            .get(self.ix)
            .map(|l| {
                self.ix += 1;
                l.clone()
            })
    }
}
use super::List;
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
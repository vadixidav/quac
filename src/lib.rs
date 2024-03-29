//! Quac is a quick-access language. The language is designed around the idea of navigating
//! through submenus and reorganizing the layout of the programming environment with more
//! specialized coding submenus. The language should feel like crafting an environment and an
//! experience, rather than coding a program. The programmer should feel like they are in their own
//! home, as if they know where everything is instinctively, in which muscle memory
//! allows you to grab the things you need right now or in the near future and put them closer to
//! your working environment.
//!
//! Quac only has one primitive: lists. What do lists have in them? One can only find out by looking
//! inside! Lists are evaluated lazily (on access), and they evaluate to more lists. Lists could
//! evaluate to either no more lists, or an infinite amount of lists. Both of these are supported.
//!
//! Accessing incredibly deep lists over and over would be inefficient and time consuming. Normal
//! programming languages attempt to use modularized APIs to constrain things into subcontainers.
//! Quac allows organization by linking. Linking takes one list and gives it to another list.
//! Containers are lists which take those lists and allow access to them in the future.
//!
//! Lists can also do things, rather than just show things. Technically, a list does things when
//! it is accessed, because it can compute things to show to the user lazily, but the list can't
//! actually change anything in this way. However, when a list is linked to another list, the list
//! it was linked to has the opportunity to do things. Since Quac itself doesn't mandate that an
//! implementation allow lists to change things, here is an example to consider: If an empty list is
//! linked to any particular list, that list can then link empty lists to all of the lists contained
//! in it, and then each of those lists can have specialized link implementations which make them
//! change the environment. The system is flexible such that execution could be handled through
//! another mechanism other than linking lists, but linking lists must be the method by which an
//! interacting user initiates execution of a task. The programmer only interacts with the system by
//! accessing and linking lists.
#![feature(core_intrinsics)]

extern crate scell;

mod container;
pub use container::Container;

use scell::SCell;
use std::any::{TypeId, Any};
use std::intrinsics::type_name;

/// The only primitive type of Quac. Every List implementation should be aware that it can be
/// accessed from anywhere and be moved anywhere.
pub trait List {
    /// Returns lazily-evaluted iterator over lists.
    fn access(&self) -> Box<Iterator<Item = SCell<List>>>;
    /// Links another list to this list.
    fn link(&mut self, other: SCell<List>);

    /// Acquire an inner type based on TypeId.
    fn acquire(&self, typeid: TypeId) -> Option<&Any>;
    /// Acquire an inner type based on TypeId.
    fn acquire_mut(&mut self, typeid: TypeId) -> Option<&mut Any>;
}

pub trait Intercast: List {
    /// Downcast a List like Any does, except allow downcasting to multiple types.
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.acquire(TypeId::of::<T>())
            .map(|any| {
                any.downcast_ref::<T>()
                    .unwrap_or_else(
                        || panic!("{}::acquire() didn't provide the correct type {}",
                               unsafe{type_name::<Self>()},
                                unsafe{type_name::<T>()}
                        )
                    )
            })
    }

    /// Downcast a List mutably like Any does, except allow downcasting to multiple types.
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.acquire_mut(TypeId::of::<T>())
            .map(|any| {
                any.downcast_mut::<T>()
                    .unwrap_or_else(
                        || panic!("{}::acquire_mut() didn't provide the correct type {}",
                               unsafe{type_name::<Self>()},
                               unsafe{type_name::<T>()}
                        )
                    )
            })
    }
}

impl<T> Intercast for T where T: List {}

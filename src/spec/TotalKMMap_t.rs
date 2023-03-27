#![allow(unused_imports)]

use builtin_macros::*;
use builtin::*;
use crate::pervasive::prelude::*;

use crate::spec::Messages_t::*;

verus!{

pub type Key = int;  // TODO: this is a placeholder for the Key type

pub type TotalKMMap = Map<Key, Message>;

pub open spec fn empty_total_map() -> Map<Key, Message> {
    // TODO: This body is a placeholder
    // TODO(verus): Should not have to declare binder twice.
    Map::new(
        |k: Key| true,
        |k: Key| Message::empty(),
    )
}

pub open spec fn total_domain() -> Set<Key>
{
    Set::new(|k:Key| true)
}

impl TotalKMMap
{
    pub open spec fn wf(self) -> bool
    {
        self.dom() == total_domain()
    }
}

}

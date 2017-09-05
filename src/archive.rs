use std::rc::Rc;
use std::cell::RefCell;

use storage::ResourceStorage;

pub trait Archive {
    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Self;
    fn is_open(&self) -> bool;
    fn describe(&self) -> &'static str;
    fn name() -> &'static str;
    fn schema() -> &'static str;
}

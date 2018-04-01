use std::fmt;
use std::marker;
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq)]
pub struct Handle<'a, T: 'a> {
    data: T,
    _phantom: marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> Handle<'a, T> {
    pub fn new(t: T) -> Self {
        Self {
            data: t,
            _phantom: marker::PhantomData,
        }
    }
}

impl<'a, T> Deref for Handle<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for Handle<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

pub struct HandleMut<'a, T: 'a> {
    data: T,
    _phantom: marker::PhantomData<&'a mut T>,
}

impl<'a, T: 'a> HandleMut<'a, T> {
    pub fn new(t: T) -> Self {
        Self {
            data: t,
            _phantom: marker::PhantomData,
        }
    }
}

impl<'a, T> Deref for HandleMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<'a, T> DerefMut for HandleMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for HandleMut<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

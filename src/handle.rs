use std::fmt;
use std::marker;
use std::ops::{Deref, DerefMut};

/// Handle to a flatdata struct for read-only fields access.
///
/// Wraps a flatdata struct and provides a read-only access to its data. A handle automatically
/// `deref`'s to a reference of the underlying struct. It is used to track the lifetime of the
/// wrapped object, since a generated flatdata struct does not tracks the lifetime of the underlying
/// data itself. A handle is similar to const references, it is possible to have several of them in
/// scope, but not a const and mutable handle at the same time, cf. [`HandleMut`].
///
/// [`HandleMut`]: struct.HandleMut.html
#[derive(Clone, PartialEq)]
pub struct Handle<'a, T: 'a> {
    data: T,
    _phantom: marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> Handle<'a, T> {
    /// Creates a new handle wrapping the given object.
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

/// Mutable handle to a flatdata struct for reading and writing fields.
///
/// Wraps a flatdata struct and provides a read and write access to its data. A handle automatically
/// `deref`'s to a reference of the underlying struct, since a generated flatdata struct does not
/// tracks the lifetime of the underlying data itself. It is used to track the lifetime of the
/// wrapped object. Similar to mutable references, it is possible to have only one mutable handle in
/// scope, and no other mutable or const handles, cf. [`Handle`].
///
/// [`Handle`]: struct.Hande.html
pub struct HandleMut<'a, T: 'a> {
    data: T,
    _phantom: marker::PhantomData<&'a mut T>,
}

impl<'a, T: 'a> HandleMut<'a, T> {
    /// Creates a new mutable handle wrapping the given object.
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

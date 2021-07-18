// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! RAII COM-pointer wrapper
use std::fmt::{self, Debug};
use ::std::ops::{Deref, DerefMut};

use winapi::um::unknwnbase::IUnknown;

#[derive(PartialEq, Eq)]
pub struct ComPtr<T> {
    ptr: *mut T,
}

impl<T> ComPtr<T> {
    /// Obtain the `ptr` and gain exclusive ownership to it.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `ptr` is a valid free COM pointer,
    /// and that it would not be used elsewhere afterwards.
    #[inline]
    pub unsafe fn new(ptr: *mut T) -> ComPtr<T> {
        debug_assert!(!ptr.is_null());
        ComPtr{ptr}
    }

    /// return the underlying raw pointer. this method should
    /// only be used for FFI
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// return the underlying mutable raw pointer. this method
    /// should only be used for FFI
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr
    }
}

// impl<T> AsRef<*mut T> for ComPtr<T> {
//     #[inline]
//     fn as_ref(&self) -> &*mut T {
//         &self.ptr
//     }
// }

// impl<T> AsMut<*mut T> for ComPtr<T> {

// }

impl<T> Deref for ComPtr<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe{&*self.ptr}
    }
}

impl<T> DerefMut for ComPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe{&mut *self.ptr}
    }
}

impl<T> Clone for ComPtr<T> {
    #[inline]
    fn clone(&self) -> ComPtr<T> {
        unsafe {as_iunknown(self.ptr).AddRef();}
        ComPtr{ptr:self.ptr}
    }
}

impl<T> Drop for ComPtr<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {as_iunknown(self.ptr).Release();}
    }
}

impl<T> Debug for ComPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = std::any::type_name::<T>();
        f.debug_struct("ComPtr") // FIXME: D3DType is not impliement for debug.
         .field(name,&self.as_ptr())
         .finish()
    }
}

#[inline]
unsafe fn as_iunknown<'a, T>(ptr: *mut T) -> &'a mut IUnknown {
    &mut *(ptr as *mut IUnknown)
}

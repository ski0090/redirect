// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! formats used by direct3d

use winapi::{shared::{dxgiformat::DXGI_FORMAT, minwindef::{BOOL, FALSE, TRUE}, ntdef::WCHAR}, um::d3d12::D3D12_RECT};

pub type DxgiFormat = DXGI_FORMAT;
pub type Rect = D3D12_RECT;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Viewport {
    pub tlx: f32,
    pub tly: f32,
    pub width: f32,
    pub height: f32,
    pub mindepth: f32,
    pub maxdepth: f32,
}

/// a 3D box
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Box3u {
    pub left: u32, pub top: u32, pub front: u32,
    pub right: u32, pub bottom: u32, pub back: u32
}

impl Viewport {
    #[inline]
    pub fn new(width: f32, height: f32) -> Viewport {
        Viewport{
            tlx: 0.0f32, tly: 0.0f32, width, height, mindepth: 0.0f32, maxdepth: 1.0f32,
        }
    }
}

/// ffi for win32 boolean values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Bool{inner: BOOL}

impl Bool {
    #[inline]
    pub fn from_win_bool(b: BOOL) -> Self{
        debug_assert!(b== TRUE || b== FALSE);
        Bool{inner:b}
    }

    #[inline]
    pub fn to_win_bool(self) -> BOOL {
        self.inner
    }

    #[inline]
    pub fn is_true(self) -> bool {
        self.inner == TRUE
    }
}

impl From<bool> for Bool {
    #[inline]
    fn from(v: bool) -> Bool {
        if v {
            Bool{inner: TRUE}
        } else {
            Bool{inner: FALSE}
        }
    }
}

impl From<Bool> for BOOL {
    #[inline]
    fn from(v: Bool) -> Self {
        v.inner
    }
}

/// convert a possibly null ended `[WCHAR]` into a `OsString`
#[inline]
pub fn from_wchar_slice(chars: &[WCHAR]) -> ::std::ffi::OsString {
    let mut end = chars.len();
    for (i, wchar) in chars.iter().enumerate() {
        if *wchar == 0 {
            end = i;
            break;
        }
    }
    <::std::ffi::OsString as ::std::os::windows::ffi::OsStringExt>::from_wide(&chars[..end])
}

// /// a data blob
// #[derive(Clone, Debug)]
// pub struct DataBlob {
//     pub(crate) ptr: ComPtr<::winapi::ID3DBlob>,
// }

// impl DataBlob {
//     /// get pointer to the underlying data
//     pub fn get_buffer_pointer(&mut self)
// }

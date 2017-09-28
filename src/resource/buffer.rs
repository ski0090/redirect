// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Various types of safe buffers

use super::raw::*;
use super::traits::*;
use device::Device;
use error::WinError;
use super::heap::traits::Heap;

/// a committed buffer with GPU-only access
#[derive(Debug)]
pub struct DefaultBuffer {
    raw: RawResource,
    size: u64,
}

impl DefaultBuffer {
    /// initial state is generic read    
    #[inline]
    pub fn new(device: &mut Device, size: u64) -> Result<DefaultBuffer, WinError> {
        let raw = device.create_committed_resource(
            &Default::default(),
            Default::default(), // TODO: check if additional denies helps?
            &super::description::ResourceDesc::buffer(size, Default::default()),
            Default::default() // TODO: other initial states?
        )?;
        Ok(DefaultBuffer{raw, size})
    }
}

impl_as_raw!(Resource, DefaultBuffer, RawResource);
unsafe impl Buffer for DefaultBuffer {
    #[inline]
    fn get_size(&self) -> u64 { self.size }
}
unsafe impl GpuOnly for DefaultBuffer {}

/// a committed buffer with cpu-write access
#[derive(Debug)]
pub struct UploadBuffer {
    raw: RawResource,
    size: u64,
}

impl UploadBuffer {
    /// initial state is generic read    
    #[inline]
    pub fn new(device: &mut Device, size: u64) -> Result<UploadBuffer, WinError> {
        let raw = device.create_committed_resource(
            &super::heap::description::HeapProperties::new(
                super::heap::description::HEAP_TYPE_UPLOAD
            ),
            Default::default(), // TODO: check if additional denies helps?
            &super::description::ResourceDesc::buffer(size, Default::default()),
            Default::default()
        )?;
        Ok(UploadBuffer{raw, size})
    }
}

impl_as_raw!(Resource, UploadBuffer, RawResource);
unsafe impl Buffer for UploadBuffer {
    #[inline]
    fn get_size(&self) -> u64 { self.size }
}
unsafe impl Upload for UploadBuffer {}

/// a committed buffer with cpu-read access
#[derive(Debug)]
pub struct ReadbackBuffer {
    raw: RawResource,
    size: u64,
}

impl ReadbackBuffer {
    /// initial state is copy dest
    #[inline]
    pub fn new(device: &mut Device, size: u64) -> Result<ReadbackBuffer, WinError> {
        let raw = device.create_committed_resource(
            &super::heap::description::HeapProperties::new(
                super::heap::description::HEAP_TYPE_READBACK
            ),
            Default::default(), // TODO: check if additional denies helps?
            &super::description::ResourceDesc::buffer(size, Default::default()),
            super::state::RESOURCE_STATE_COPY_DEST
        )?;
        Ok(ReadbackBuffer{raw, size})
    }
}

impl_as_raw!(Resource, ReadbackBuffer, RawResource);
unsafe impl Buffer for ReadbackBuffer {
    #[inline]
    fn get_size(&self) -> u64 { self.size }
}
unsafe impl Readback for ReadbackBuffer {}

#[derive(Debug)]
pub struct PlacedBuffer<H> {
    raw: RawResource,
    heap: H,
    /// offset from heap start
    offset: u64,
}

impl<H: Heap> Resource for PlacedBuffer<H> {
    #[inline]
    fn as_raw(&self) -> &RawResource {
        &self.raw
    }

    #[inline]
    fn as_raw_mut(&mut self) -> &mut RawResource {
        &mut self.raw
    }
}

unsafe impl<H: Heap> Placed for PlacedBuffer<H> {
    type Heap = H;

    #[inline]
    fn get_placed_heap(&mut self) -> &mut Self::Heap {
        &mut self.heap
    }

    #[inline]
    fn get_heap_offset(&self) -> u64 {
        self.offset
    }
}

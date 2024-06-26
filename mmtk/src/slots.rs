//slots.rs wip
extern crate atomic;
use self::atomic::Atomic;
use mmtk::util::constants::LOG_BYTES_IN_ADDRESS;
use mmtk::{
    util::{Address, ObjectReference},
    vm::slot::{MemorySlice, SimpleSlot, Slot},
};

// The simplest case is use `mmtk::vm::SimpleSlot`: https://docs.mmtk.io/api/mmtk/vm/slot/struct.SimpleSlot.html
// If a VM supports multiple kinds of slots, we can use tagged union to represent all of them.
// This implementation here is just examples. The binding should only keep what they need, and rewrite the implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DummyVMSlot {
    Simple(SimpleSlot),
    #[cfg(target_pointer_width = "64")]
    Compressed(only_64_bit::CompressedOopSlot),
    Offset(OffsetSlot),
    Tagged(TaggedSlot),
}

unsafe impl Send for DummyVMSlot {}

impl Slot for DummyVMSlot {
    fn load(&self) -> Option<ObjectReference> {
        match self {
            DummyVMSlot::Simple(e) => e.load(),
            #[cfg(target_pointer_width = "64")]
            DummyVMSlot::Compressed(e) => e.load(),
            DummyVMSlot::Offset(e) => e.load(),
            DummyVMSlot::Tagged(e) => e.load(),
        }
    }

    fn store(&self, object: ObjectReference) {
        match self {
            DummyVMSlot::Simple(e) => e.store(object),
            #[cfg(target_pointer_width = "64")]
            DummyVMSlot::Compressed(e) => e.store(object),
            DummyVMSlot::Offset(e) => e.store(object),
            DummyVMSlot::Tagged(e) => e.store(object),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DummyVMMemorySlice(*mut [ObjectReference]);

unsafe impl Send for DummyVMMemorySlice {}

impl MemorySlice for DummyVMMemorySlice {
    type SlotType = DummyVMSlot;
    type SlotIterator = DummyVMMemorySliceIterator;

    fn iter_slots(&self) -> Self::SlotIterator {
        DummyVMMemorySliceIterator {
            cursor: unsafe { (*self.0).as_mut_ptr_range().start },
            limit: unsafe { (*self.0).as_mut_ptr_range().end },
        }
    }

    fn object(&self) -> Option<ObjectReference> {
        None
    }

    fn start(&self) -> Address {
        Address::from_ptr(unsafe { (*self.0).as_ptr_range().start })
    }

    fn bytes(&self) -> usize {
        unsafe { std::mem::size_of_val(&*self.0) }
    }

    fn copy(src: &Self, tgt: &Self) {
        debug_assert_eq!(src.bytes(), tgt.bytes());
        debug_assert_eq!(
            src.bytes() & ((1 << LOG_BYTES_IN_ADDRESS) - 1),
            0,
            "bytes are not a multiple of words"
        );
        // Raw memory copy
        unsafe {
            let words = tgt.bytes() >> LOG_BYTES_IN_ADDRESS;
            let src = src.start().to_ptr::<usize>();
            let tgt = tgt.start().to_mut_ptr::<usize>();
            std::ptr::copy(src, tgt, words)
        }
    }
}

pub struct DummyVMMemorySliceIterator {
    cursor: *mut ObjectReference,
    limit: *mut ObjectReference,
}

impl Iterator for DummyVMMemorySliceIterator {
    type Item = DummyVMSlot;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.limit {
            None
        } else {
            let edge = self.cursor;
            self.cursor = unsafe { self.cursor.add(1) };
            Some(DummyVMSlot::Simple(SimpleSlot::from_address(
                Address::from_ptr(edge),
            )))
        }
    }
}

/// Compressed OOP slot only makes sense on 64-bit architectures.
#[cfg(target_pointer_width = "64")]
pub mod only_64_bit {
    use super::*;

    /// This represents a location that holds a 32-bit pointer on a 64-bit machine.
    ///
    /// OpenJDK uses this kind of slot to store compressed OOPs on 64-bit machines.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct CompressedOopSlot {
        slot_addr: *mut Atomic<u32>,
    }

    unsafe impl Send for CompressedOopSlot {}

    impl CompressedOopSlot {
        pub fn from_address(address: Address) -> Self {
            Self {
                slot_addr: address.to_mut_ptr(),
            }
        }
        pub fn as_address(&self) -> Address {
            Address::from_mut_ptr(self.slot_addr)
        }
    }

    impl Slot for CompressedOopSlot {
        fn load(&self) -> Option<ObjectReference> {
            let compressed = unsafe { (*self.slot_addr).load(atomic::Ordering::Relaxed) };
            let expanded = (compressed as usize) << 3;
            ObjectReference::from_raw_address(unsafe { Address::from_usize(expanded) })
        }

        fn store(&self, object: ObjectReference) {
            let expanded = object.to_raw_address().as_usize();
            let compressed = (expanded >> 3) as u32;
            unsafe { (*self.slot_addr).store(compressed, atomic::Ordering::Relaxed) }
        }
    }
}

/// This represents an slot that holds a pointer to the *middle* of an object, and the offset is known.
///
/// Julia uses this trick to facilitate deleting array elements from the front.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OffsetSlot {
    slot_addr: *mut Atomic<Address>,
    offset: usize,
}

unsafe impl Send for OffsetSlot {}

impl OffsetSlot {
    pub fn new_no_offset(address: Address) -> Self {
        Self {
            slot_addr: address.to_mut_ptr(),
            offset: 0,
        }
    }

    pub fn new_with_offset(address: Address, offset: usize) -> Self {
        Self {
            slot_addr: address.to_mut_ptr(),
            offset,
        }
    }

    pub fn slot_address(&self) -> Address {
        Address::from_mut_ptr(self.slot_addr)
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl Slot for OffsetSlot {
    fn load(&self) -> Option<ObjectReference> {
        let middle = unsafe { (*self.slot_addr).load(atomic::Ordering::Relaxed) };
        let begin = middle - self.offset;
        ObjectReference::from_raw_address(begin)
    }

    fn store(&self, object: ObjectReference) {
        let begin = object.to_raw_address();
        let middle = begin + self.offset;
        unsafe { (*self.slot_addr).store(middle, atomic::Ordering::Relaxed) }
    }
}

/// This slot presents the object reference itself to mmtk-core.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaggedSlot {
    slot_addr: *mut Atomic<usize>,
}

unsafe impl Send for TaggedSlot {}

impl TaggedSlot {
    // The DummyVM has OBJECT_REF_OFFSET = 4.
    // Using a two-bit tag should be safe on both 32-bit and 64-bit platforms.
    const TAG_BITS_MASK: usize = 0b11;

    pub fn new(address: Address) -> Self {
        Self {
            slot_addr: address.to_mut_ptr(),
        }
    }
}

impl Slot for TaggedSlot {
    fn load(&self) -> Option<ObjectReference> {
        let tagged = unsafe { (*self.slot_addr).load(atomic::Ordering::Relaxed) };
        let untagged = tagged & !Self::TAG_BITS_MASK;
        ObjectReference::from_raw_address(unsafe { Address::from_usize(untagged) })
    }

    fn store(&self, object: ObjectReference) {
        let old_tagged = unsafe { (*self.slot_addr).load(atomic::Ordering::Relaxed) };
        let new_untagged = object.to_raw_address().as_usize();
        let new_tagged = new_untagged | (old_tagged & Self::TAG_BITS_MASK);
        unsafe { (*self.slot_addr).store(new_tagged, atomic::Ordering::Relaxed) }
    }
}
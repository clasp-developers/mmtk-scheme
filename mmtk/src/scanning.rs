//scanning.rs wip
use crate::slots::DummyVMSlot;
use crate::DummyVM;
use UPCALLS;
use MyStruct;

use std::io;
use std::io::Write;
use mmtk::util::opaque_pointer::*;
use mmtk::util::ObjectReference;
use mmtk::vm::RootsWorkFactory;
use mmtk::vm::Scanning;
use mmtk::vm::SlotVisitor;
use mmtk::Mutator;
use object_model::OBJECT_REF_OFFSET;
use mmtk::util::metadata::side_metadata::VO_BIT_SIDE_METADATA_ADDR;
use mmtk::util::Address;
use std::mem::size_of;
use mmtk::memory_manager::is_mmtk_object;



pub struct VMScanning {}

// Documentation: https://docs.mmtk.io/api/mmtk/vm/scanning/trait.Scanning.html
impl Scanning<DummyVM> for VMScanning {
    fn scan_roots_in_mutator_thread(
        _tls: VMWorkerThread,
        _mutator: &'static mut Mutator<DummyVM>,
        _factory: impl RootsWorkFactory<DummyVMSlot>,
    ) {
        let value: usize = 42; // Declare a usize value on the stack
        let stack_top: *const usize = unsafe { ((*UPCALLS).mutator_stack_top)(_mutator) };
        let stack_bottom: *const usize = &value; // Take its address

        // Print the value and its address
        println!("In scan_roots_in_mutator_thread -----------------------------------");
        println!("   Stack top: {:p}", stack_top);
        println!("Stack bottom: {:p}", stack_bottom);

        unimplemented!()
    }
    fn scan_vm_specific_roots(_tls: VMWorkerThread, _factory: impl RootsWorkFactory<DummyVMSlot>) {
        println!("scan_vm_specific_roots");
        let num_entries_in_sptab : i32 = unsafe { ((*UPCALLS).num_entries_in_sptab)() };
        let num_entries_in_isymtab : i32 = unsafe { ((*UPCALLS).num_entries_in_isymtab)() };
        let first_in_sptab : *mut MyStruct = unsafe { ((*UPCALLS).first_in_sptab)()};
        let first_in_isymtab: *mut MyStruct = unsafe { ((*UPCALLS).first_in_isymtab)()};
        // Write a for loop walking a pointer from first_in_sptab for num_entries_in_sptab and put the second slot in a vec


        unimplemented!()
    }
    fn scan_object<EV: SlotVisitor<DummyVMSlot>>(
        _tls: VMWorkerThread,
        _object: ObjectReference,
        _slot_visitor: &mut EV,
    ) {
        println!("scan_object");
        unimplemented!()
    }
    fn notify_initial_thread_scan_complete(_partial_scan: bool, _tls: VMWorkerThread) {
        println!("notify_initial_thread_scan_complete");
        unimplemented!()
    }
    fn supports_return_barrier() -> bool {
        println!("supports_return_barrier");
        unimplemented!()
    }
    fn prepare_for_roots_re_scanning() {
        println!("prepare_for_roots_re_scanning");
        unimplemented!()
    }
}



//try_pointer function written by Adel Prokurov
//source: https://mmtk.zulipchat.com/#narrow/stream/262679-General/topic/.E2.9C.94.20Finding.20object.20start.20using.20VO.20bits/near/402951366

//define simple round_down function for the find_pointer function
fn round_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

/// Checks if `pointer` is a valid object in MMTk heap.
///
/// `pointer` can be an interior pointer in which case we search for first vo_bit set to `1`
/// and return object that corresponds to that bit.
///
/// Return value is `pointer + OBJECT_REF_OFFSET` because `vo_bit` is set for `alloc()` result
/// while our actual object references start at `alloc() + OBJECT_REF_OFFSET`.
pub unsafe fn try_pointer(base: usize, pointer: usize, vo_bits: usize) -> Option<usize> {
    /// Limit of interior pointer offsets
    ///
    /// This is the maximum offset before we stop search for object start.
    const INTERIOR_LIMIT: usize = 512;
    let mut obj = round_down(pointer, size_of::<usize>() as _);
    let from_base_obj = obj - base;
    let mut bits_since_start = from_base_obj / (size_of::<usize>() * 8);
    let mut vo_bit_byte_addr = vo_bits + bits_since_start / 8;
    let mut vo_bit_in_byte_shift = bits_since_start % 8;
    let mut covered = 0;
    let mut byte_val = (vo_bit_byte_addr as *const u8).read();

    while (byte_val >> vo_bit_in_byte_shift) & 1 == 0 {
        obj -= size_of::<usize>();
        covered += 8;
        bits_since_start = obj / (size_of::<usize>() * 8);
        vo_bit_byte_addr = vo_bits.wrapping_add(bits_since_start / 8);
        vo_bit_in_byte_shift = bits_since_start % 8;

        if vo_bit_byte_addr < vo_bits || covered > INTERIOR_LIMIT {
            return None;
        }

        byte_val = (vo_bit_byte_addr as *const u8).read();
    }

    if (byte_val >> vo_bit_in_byte_shift) & 1 == 0 {
        None
    } else {
        Some(obj + OBJECT_REF_OFFSET as usize)
    }
}

unsafe fn try_stack_word(
    cursor: *mut *mut u8,
    addr: usize,
    base: usize,
    end: usize,
    vo_bits: usize, 
//    roots: &mut Vec<ObjectReference>,
) {
    if base <= addr && addr < end {
        if let Some(addr) = try_pointer( base, addr, vo_bits ) {
            // let obj = ObjectReference::from_raw_address(Address::from_usize(addr));
            //            roots.push(obj);
            println!("Stack pointer at {:#x} -> {:#x}", cursor as usize, addr );
        }
    }
}

pub unsafe fn process_stack(
//    factory: &mut impl RootsWorkFactory<ScmSlot>,
    begin: *mut *mut u8,
    stack_end:*mut *mut u8,
) {
    println!("In process_stack");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut cursor = begin;
//    let stack_end = begin.add(size) as *mut *mut u8;
//    let mut roots = Vec::with_capacity(64);
    let heap_base = mmtk::memory_manager::starting_heap_address().as_usize();
    let heap_end = mmtk::memory_manager::last_heap_address().as_usize();

    let vo_bits = VO_BIT_SIDE_METADATA_ADDR.as_usize();
    while cursor < stack_end {
        let word = cursor.read();
        is_mmtk_object(Address::from_ptr(word));
        try_stack_word(cursor, word as usize, heap_base, heap_end, vo_bits ); // , &mut roots);
        cursor = cursor.add(1);
    }
//    factory.create_process_pinning_roots_work(roots);
}


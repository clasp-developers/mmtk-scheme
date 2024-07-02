extern crate libc; 
extern crate mmtk;
extern crate lazy_static;

use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;

use mmtk::util::opaque_pointer::*;
use mmtk::Mutator;
use mmtk::vm::VMBinding;
use mmtk::MMTK;

pub mod active_plan;
pub mod api;
pub mod collection;
pub mod object_model;
pub mod reference_glue;
pub mod scanning;

use std::ptr::null_mut;

mod slots;

pub type DummyVMSlot = mmtk::vm::slot::SimpleSlot;

#[derive(Default)]
pub struct DummyVM;

//documentation comment
impl VMBinding for DummyVM {
    type VMObjectModel = object_model::VMObjectModel;
    type VMScanning = scanning::VMScanning;
    type VMCollection = collection::VMCollection;
    type VMActivePlan = active_plan::VMActivePlan;
    type VMReferenceGlue = reference_glue::VMReferenceGlue;
    type VMSlot = slots::DummyVMSlot;
    type VMMemorySlice = slots::DummyVMMemorySlice;

    //allowed maximum alignment in bytes
    //const MAX_ALIGNMENT: usize = 1 << 6;
}

pub static MMTK_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub static SINGLETON: OnceLock<Box<MMTK<DummyVM>>> = OnceLock::new();

//access singleton
fn mmtk() -> &'static MMTK<DummyVM> {
   SINGLETON.get().unwrap() 
}

//upcalls
#[repr(C)]
pub struct scheme_Upcalls {
    //a bunch of functions to go in here
    pub mutator_stack_top: extern "C" fn(*mut Mutator<DummyVM>) -> *const usize,
    pub block_for_gc: extern "C" fn(VMMutatorThread),
}

pub static mut UPCALLS: *const scheme_Upcalls = null_mut();

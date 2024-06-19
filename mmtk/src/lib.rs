//theirs lowkey

extern crate libc; 
extern crate mmtk;
extern crate lazy_static;

use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;

use mmtk::vm::VMBinding;
use mmtk::MMTK;

#[derive(Default)]
pub struct Scheme;

pub mod active_plan;
pub mod api;
pub mod collection;
pub mod object_model;
pub mod reference_glue;
pub mod scanning;

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
pub fn mmtk() -> &'static MMTK<DummyVM> {
   SINGLETON.get().unwrap() 
}

extern crate libc; 
extern crate mmtk;
extern crate lazy_static;
use lazy_static::lazy_static;

use libc::{c_char, c_void};

use std::sync::{OnceLock, Condvar, Mutex};
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

#[repr(C)]
pub struct MyStruct {
    c_string: *const c_char,
    void_ptr: *mut c_void,
}


//upcalls
#[repr(C)]
pub struct scheme_Upcalls {
    //a bunch of functions to go in here
    pub mutator_stack_top: extern "C" fn(*mut Mutator<DummyVM>) -> *const usize,
    pub stop_all_mutators: extern "C" fn(VMWorkerThread),
    pub resume_mutators: extern "C" fn(VMWorkerThread),
    pub get_mutators: extern "C" fn(
        visit_mutator: extern "C" fn(*mut Mutator<DummyVM>, *mut libc::c_void),
        data: *mut libc::c_void,
    ),
    pub scan_vm_specific_roots: extern "C" fn(VMWorkerThread),
    pub num_entries_in_sptab: extern "C" fn() -> i32,
    pub num_entries_in_isymtab: extern "C" fn() -> i32,
    pub first_in_sptab: unsafe extern "C" fn() -> *mut MyStruct,
    pub first_in_isymtab: unsafe extern "C" fn() -> *mut MyStruct,
    
}

pub static mut UPCALLS: *const scheme_Upcalls = null_mut();

struct MutatorStatus {
    is_running: bool,
}

lazy_static!{
    static ref MUTATOR_STATUS: (Mutex<MutatorStatus>, Condvar) = (
    	Mutex::new(MutatorStatus {is_running : true}),
	Condvar::new()
    );
}

//lib.rs wip
extern crate libc; //new
extern crate mmtk;
extern crate lazy_static; //new
use lazy_static::lazy_static;


//all new
use mmtk::vm::VMBinding;
use mmtk::MMTKBuilder; //used for lazy_static
use mmtk::MMTK; //used for lazy_static

pub mod active_plan; //new
//pub mod api; //new, must stay commented out else compilation fails 6/4/2024
pub mod collection; //new
pub mod object_model;
pub mod reference_glue; //new
pub mod scanning;

mod slots;


#[derive(Default)]
pub struct DummyVM;

/// The edge type of 

impl VMBinding for DummyVM {
    // Implement required VMBinding traits here
    type VMObjectModel = object_model::VMObjectModel;
    type VMScanning = scanning::VMScanning;
    type VMCollection = collection::VMCollection;
    type VMActivePlan = active_plan::VMActivePlan;
    type VMReferenceGlue = reference_glue::VMReferenceGlue;
    type VMSlot = slots::DummyVMSlot;
    type VMMemorySlice = slots::DummyVMMemorySlice;
}

//standard libraries
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex; //used for lazy_static


//github to find initializeonce: mmtk-core/src/util/rust_util/mod.rs
/*
use mmtk::util::InitializeOnce;
pub static SINGLETON: InitializeOnce<Box<MMTK<DummyVM>>> = InitializeOnce::new();


#[no_mangle]
pub extern "C" fn initialize_mmtk()
{
    println!("Hello World - in initialize_mmtk();");
    use mmtk::util::options::PlanSelector;
    // set heap size
    let mut builder = mmtk::MMTKBuilder::new();
    let _success =
        builder
                .options
                .gc_trigger
                .set(mmtk::util::options::GCTriggerSelector::FixedHeapSize(1024*1024*256,));
    let plan = PlanSelector::NoGC;
    let _success2 = builder.options.plan.set(plan);
    let _mmtk = mmtk::memory_manager::mmtk_init::<DummyVM>(&builder);
}
*/
pub static MMTK_INITIALIZED: AtomicBool = AtomicBool::new(false);

lazy_static! {
    pub static ref BUILDER: Mutex<MMTKBuilder> = Mutex::new(MMTKBuilder::new());
    pub static ref SINGLETON: MMTK<DummyVM> = {
        let builder = BUILDER.lock().unwrap();
        debug_assert!(!MMTK_INITIALIZED.load(Ordering::SeqCst));
        let ret = mmtk::memory_manager::mmtk_init(&builder);
        MMTK_INITIALIZED.store(true, std::sync::atomic::Ordering::Relaxed);
        *ret
    };
}


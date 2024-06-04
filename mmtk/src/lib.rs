//lib.rs wip
extern crate libc; //new
extern crate mmtk;
extern crate lazy_static; //new

//all new
use mmtk::vm::VMBinding;
//use mmtk::MMTKBuilder;
//use mmtk::MMTK;

pub mod active_plan; //new
//pub mod api; //new
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
//use std::sync::atomic::{AtomicBool, Ordering};
//use std::sync::Mutex;

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
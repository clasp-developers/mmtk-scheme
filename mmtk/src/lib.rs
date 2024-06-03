extern crate mmtk;

pub mod object_model;
pub mod scanning;

#[derive(Default)]
pub struct DummyVM;

/// The edge type of 

impl mmtk::vm::VMBinding for DummyVM {
    // Implement required VMBinding traits here
    // Compilation fails because I'm missing implementations
    type VMObjectModel = object_model::VMObjectModel;
    type VMScanning = scanning::VMScanning;
}

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
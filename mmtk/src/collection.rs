use crate::DummyVM;
use mmtk::util::opaque_pointer::*;
use mmtk::vm::Collection;
use mmtk::vm::GCThreadContext;
use mmtk::Mutator;
use SINGLETON;
//use std::thread;

pub struct VMCollection {}

// Documentation: https://docs.mmtk.io/api/mmtk/vm/collection/trait.Collection.html
impl Collection<DummyVM> for VMCollection {
    fn stop_all_mutators<F>(_tls: VMWorkerThread, _mutator_visitor: F)
    where
        F: FnMut(&'static mut Mutator<DummyVM>),
    {
        unimplemented!()
    }

    fn resume_mutators(_tls: VMWorkerThread) {
        unimplemented!()
    }

    /// Block the current thread for GC. This is called when an allocation request cannot be fulfilled and a GC
    /// is needed. MMTk calls this method to inform the VM that the current thread needs to be blocked as a GC
    /// is going to happen. Then MMTk starts a GC. For a stop-the-world GC, MMTk will then call `stop_all_mutators()`
    /// before the GC, and call `resume_mutators()` after the GC.
    ///
    /// Arguments:
    /// * `tls`: The current thread pointer that should be blocked. The VM can optionally check if the current thread matches `tls`.
    fn block_for_gc(_tls: VMMutatorThread) {
        println!("block_for_gc: implement me fully");
        //unimplemented!()
    }

    fn spawn_gc_thread(_tls: VMThread, ctx: GCThreadContext<DummyVM>) {
        // Just drop the join handle. The thread will run until the process quits.
        println!("In spawn_gc_thread - spawning thread");
        let _ = std::thread::spawn(move || {
            use mmtk::util::opaque_pointer::*;
            use mmtk::util::Address;
            let worker_tls = VMWorkerThread(VMThread(OpaquePointer::from_address(unsafe {
                Address::from_usize(thread_id::get())
            })));
            match ctx {
                GCThreadContext::Worker(w) => {
                    mmtk::memory_manager::start_worker(&SINGLETON.get().unwrap(), worker_tls, w)
                }
            }
    });
    }
}

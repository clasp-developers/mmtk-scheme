use crate::DummyVM;
use mmtk::util::opaque_pointer::*;
use mmtk::vm::Collection;
use mmtk::vm::GCThreadContext;
use mmtk::Mutator;
use SINGLETON;
//use UPCALLS;
use MUTATOR_STATUS;

//use std::thread;

pub struct VMCollection {}

// Documentation: https://docs.mmtk.io/api/mmtk/vm/collection/trait.Collection.html
impl Collection<DummyVM> for VMCollection {
    fn stop_all_mutators<F>(_tls: VMWorkerThread, _mutator_visitor: F)
    where
        F: FnMut(&'static mut Mutator<DummyVM>),
    {
        //unimplemented!()
	let (lock, condition) = &*MUTATOR_STATUS;
	let mut state = lock.lock().unwrap();
	state.is_running = false; //stop the mutator
	if !state.is_running{
	   println!("stop_all_mutators: success blocked mutator thread");
	} else {
	   println!("stop_all_mutators: fail");
	}
	condition.notify_all();
    }

    fn resume_mutators(_tls: VMWorkerThread) {
        let (lock, condition) = &*MUTATOR_STATUS; //get mutex & condition variable
	let mut state = lock.lock().unwrap();
	state.is_running = true; //update to resume mutator thread
	if state.is_running{
	   println!("resume_mutators: success resumed mutator thread");
	} else {
	   println!("resume_mutators: fail");
	}
	condition.notify_all();
    }

    /// Block the current thread for GC. This is called when an allocation request cannot be fulfilled and a GC
    /// is needed. MMTk calls this method to inform the VM that the current thread needs to be blocked as a GC
    /// is going to happen. Then MMTk starts a GC. For a stop-the-world GC, MMTk will then call `stop_all_mutators()`
    /// before the GC, and call `resume_mutators()` after the GC.
    ///
    /// Arguments:
    /// * `tls`: The current thread pointer that should be blocked. The VM can optionally check if the current thread matches `tls`.
    fn block_for_gc(_tls: VMMutatorThread) {
        println!("block_for_gc: starting");
	let (lock, condition) = &*MUTATOR_STATUS;
	let mut state = lock.lock().unwrap();
	while !state.is_running{
	      state = condition.wait(state).unwrap();
	}
	println!("block_for_gc: mutator resumed");
        //unsafe { ((*UPCALLS).block_for_gc)(_tls) };
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

impl VMCollection {
    extern "C" fn notify_mutator_ready<F>(mutator_ptr: *mut Mutator<DummyVM>, data: *mut libc::c_void)
    where
        F: FnMut(&'static mut mmtk::Mutator<DummyVM>),
    {
        let mutator = unsafe { &mut *mutator_ptr };
        let mutator_visitor = unsafe { &mut *(data as *mut F) };
        mutator_visitor(mutator);
    }
}

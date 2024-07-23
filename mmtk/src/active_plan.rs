use crate::DummyVM;
use mmtk::util::opaque_pointer::*;
use mmtk::vm::ActivePlan;
use mmtk::Mutator;

use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr;

pub struct VMActivePlan {}

//box needs 1 entry thats zero because we only have one mutator

struct SillyIterator<'a>{
    mutators: VecDeque<&'a mut Mutator<DummyVM>>,
    phantom_data: PhantomData<&'a ()>,
}

// Documentation: https://docs.mmtk.io/api/mmtk/vm/active_plan/trait.ActivePlan.html
impl ActivePlan<DummyVM> for VMActivePlan {
    fn number_of_mutators() -> usize {
        unimplemented!()
    }

    fn is_mutator(_tls: VMThread) -> bool {
        return true;
//        unimplemented!()
    }

    fn mutator(_tls: VMMutatorThread) -> &'static mut Mutator<DummyVM> {
        unimplemented!()
    }

    fn mutators<'a>() -> Box<dyn Iterator<Item = &'a mut Mutator<DummyVM>> + 'a> {
        let mut new_mutators = VecDeque::new();
	
        let null_mutator : *mut Mutator<DummyVM> = ptr::null_mut();
        new_mutators.push_back(unsafe { &mut *null_mutator} );

	Box::new(SillyIterator {
            mutators: new_mutators,
            phantom_data: PhantomData,
        });
	
	unimplemented!()
    }
}

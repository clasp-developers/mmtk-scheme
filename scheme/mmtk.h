#ifndef MMTK_H
#define MMTK_H

#include <stddef.h>
#include <sys/types.h>

// The extern "C" is only required if the runtime
// implementation language is C++


// C functions

void mmtk_c_test();



// An arbitrary address
  typedef void* Address;
  // MmtkMutator should be an opaque pointer for the VM
  typedef void* MmtkMutator;
  // An opaque pointer to a VMThread
  typedef void* VMThread;

  /**
   * Initialize MMTk instance
   */
  void mmtk_init();

  /**
   * Set the heap size
   *
   * @param min minimum heap size
   * @param max maximum heap size
   */
  void mmtk_set_heap_size(size_t min, size_t max);


#endif // MMTK_H

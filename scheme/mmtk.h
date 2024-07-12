#ifndef MMTK_H
#define MMTK_H

#include <stddef.h>
#include <sys/types.h>

// The extern "C" is only required if the runtime
// implementation language is C++


// C functions

void mmtk_c_test();

int mmtk_is_mmtk_object(void* addr);

// An arbitrary address
  typedef void* Address;
  // MmtkMutator should be an opaque pointer for the VM
  typedef void* MmtkMutator;
  // An opaque pointer to a VMThread
  typedef void* VMThread;

void* mmtk_create_builder();

int mmtk_set_option_from_string( void* builder, const char* name, const  char* value );

  /**
   * Initialize MMTk instance
   */
void mmtk_init(void* builder, void* rtupcalls);

  /**
   * Set the heap size
   *
   * @param min minimum heap size
   * @param max maximum heap size
   */
int mmtk_set_fixed_heap_size(void* mutator, size_t size);

void* mmtk_bind_mutator(void* mutator);

void* mmtk_alloc( void* mutator,
                  size_t size,
                  size_t align,
                  size_t offset,
                  int semantics );

void* mmtk_post_alloc( void* mutator,
                       void* obj, 
                       size_t size,
                       int semantics );


void* mmtk_destroy_mutator(void* mutator);

void mmtk_handle_user_collection_request(void* tls);
void mmtk_initialize_collection(void* tls);

void mmtk_scan_vm_specific_roots(void *workerThread, void* factory);

int mmtk_num_entries_in_sptab();
int mmtk_num_entries_in_isymtab();
void* mmtk_first_in_sptab();
void* mmtk_first_in_isymtab();


//
// Print info on mmtk
//
void mmtk_info();

void mmtk_scan_stack(void** bottom, void** top);

//upcalls
typedef struct {} scheme_Upcalls;

#endif // MMTK_H

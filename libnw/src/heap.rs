/// Initialize the heap with the size passed as the first parameter.
///
/// This macro must be called before attempting to
/// use any heap-allocated struct.
#[macro_export]
macro_rules! init_heap {
    ($size:expr) => {{
        use core::mem::MaybeUninit;
        use embedded_alloc::LlffHeap as Heap;

        /// The size of the heap.
        const HEAP_SIZE: usize = $size;

        /// The heap buffer.
        static mut HEAP_BUFFER: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

        /// The heap allocator.
        #[global_allocator]
        static HEAP: Heap = Heap::empty();

        unsafe {
            HEAP.init(&raw mut HEAP_BUFFER as usize, HEAP_SIZE);
        }
    }};
}

// These 3 functions are required by the linker
#[unsafe(no_mangle)]
extern "C" fn _critical_section_1_0_acquire() {}
#[unsafe(no_mangle)]
extern "C" fn _critical_section_1_0_release() {}
#[unsafe(no_mangle)]
extern "C" fn __aeabi_unwind_cpp_pr0() {}

/// Initializes a global heap allocator using a statically allocated buffer.
///
/// This macro sets up a global allocator using [`embedded_alloc::LlffHeap`],
/// suitable for `no_std` embedded environments such as EADK apps.
///
/// You must call this macro **before using any heap-allocated types** like `Box`, `Vec`, etc.
///
/// # Example
///
/// ```rust
/// init_heap!(1024); // Initializes a 1 KB heap
/// ```
///
/// # Safety
///
/// This macro uses `unsafe` internally to initialize the heap. You must ensure that:
///
/// - The macro is only called once.
/// - No heap allocations occur before it is called.
/// - You use it in the `main()` function or at the very start of your app.
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
            // SAFETY: the heap buffer is allocated with a size of `HEAP_SIZE`.
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

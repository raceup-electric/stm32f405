use embedded_alloc::LlffHeap as HEAP;

#[global_allocator]
static HEAP: HEAP = HEAP::empty();

pub fn init_heap(){
    use core::ptr::addr_of_mut;
    use core::mem::MaybeUninit;
    const HEAP_SIZE : usize = 1024; //change if needed
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) };
}

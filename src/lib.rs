// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![feature(alloc_error_handler)]

use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr;

pub struct NtGlobalAlloc {
    pub head: u32,
    pub end: u32,
}

unsafe impl Sync for NtGlobalAlloc {}

unsafe impl GlobalAlloc for NtGlobalAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let allocated_size = core::ptr::read_volatile(self.head as *mut u32);
        let offset = 0x100; // for header
        let align = layout.align() as u32;
        let align_offset = match allocated_size % align {
            0 => 0,
            m => align - m,
        };
        let pointer = self.head + allocated_size + offset + align_offset; // to return

        // ensure to keep requested size.
        if pointer + layout.size() as u32 > self.end {
            return ptr::null_mut();
        }

        // store next value
        core::ptr::write_volatile(
            self.head as *mut u32,
            allocated_size + (layout.size() as u32) + align_offset,
        );

        pointer as *mut u8
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }

    // unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    //     self.alloc(layout)
    // }
}

impl NtGlobalAlloc {
    pub unsafe fn init(&mut self) {
        let base = self.head;
        for i in 0..0x100 {
            core::ptr::write_volatile((base + i * 4) as *mut u32, 0);
        }
        for i in 0x101..0x10000 {
            core::ptr::write_volatile((base + i * 4) as *mut u32, i | 0xFF00_0000);
        }
    }
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}

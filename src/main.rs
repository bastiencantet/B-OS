#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use B_OS::{allocator, memory, println};

use core::arch::asm;
use core::panic::PanicInfo;
use x86_64::registers::control::Cr3;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Page;
use B_OS::memory::translate_addr;
extern crate alloc;
use raw_cpuid::CpuId;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use B_OS::cpuinfo::print_cpu_blok_info;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {

    print_cpu_blok_info();

    use x86_64::VirtAddr;
    use B_OS::memory::BootInfoFrameAllocator;

    println!("Hello World{}", "!");
    B_OS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));



    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    B_OS::hlt_loop();
}
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    B_OS::hlt_loop();
}




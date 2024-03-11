#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        //trun PIN21 into output
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);

        loop{
            //Turn Pin ON
            core::ptr::write_volatile(0x3F20001C as *mut u32, 1<<21);

            for _ in 1..50000{
                asm!("nop")
            }

            core::ptr::write_volatile(0x3F200028 as *mut u32, 1<<21);
            
            for _ in 1..50000{
                asm!("nop")
            }
        }
    }
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> !{
    loop{}
}


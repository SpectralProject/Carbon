use std::arch::asm;

// spawn a process running a program
// if get a negative number, failed. If positive, success with pid
pub fn spawn_process(path: &str, name: &str) -> i64 {
    // should make an asm syscall
    unsafe {
        asm!("nop");
    }

    0
}

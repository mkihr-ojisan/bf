pub fn run(code: &[u8]) {
    let mut memory = vec![0u8; 30000];

    unsafe {
        let executable_code = libc::mmap(
            std::ptr::null_mut(),
            code.len(),
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        std::ptr::copy_nonoverlapping(code.as_ptr(), executable_code as *mut u8, code.len());
        let f: extern "C" fn(memory: *mut u8) = std::mem::transmute(executable_code);
        f(memory.as_mut_ptr());
        libc::munmap(executable_code, code.len());
    };
}

#![feature(rustc_private)]
extern crate libc;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct LowMemoryBlock {
    start: *mut u8,
    size: usize,
}

impl LowMemoryBlock {
    pub fn as_slice(&self) -> &[u8]
    {
        unsafe {
            core::slice::from_raw_parts(
                self.start,
                self.size
            )
        }
    }

    pub fn as_slice_mut(&self) -> &mut [u8]
    {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.start,
                self.size
            )
        }
    }
}

impl LowMemoryBlock {
    pub fn new_from_slice(data: &[u8]) -> Result<Self, &'static str> {
        let mem_block = LowMemoryBlock::new(data.len());

        if !mem_block.is_ok() {
            return mem_block;
        }

        let block = mem_block.unwrap();
        block.as_slice_mut().copy_from_slice(data);

        Ok(block)
    }

    pub fn new(size: usize) -> Result<Self, &'static str> {
        if size >= 0xFFFFFFFF {
            return Err("Too big a slice");
        }
        if size == 0 {
            return Err("A memory block must contain at least 1 byte");
        }

        let prot: libc::c_int = libc::PROT_READ | libc::PROT_WRITE;
        let flags: libc::c_int = if cfg!(target_arch = "x86_64") && cfg!(target_os = "linux") {
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_32BIT
        } else {
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS
        };

        let test_addrs: [usize; 3] = [0x1000, 0x100000, 0x40000000];
        for addr in test_addrs.iter() {
            let test_addr = *addr as *mut libc::c_void;
            let pointer = unsafe { libc::mmap(test_addr, size, prot, flags, -1, 0) };
            if (pointer != 0 as *mut libc::c_void)
                && (pointer != libc::MAP_FAILED)
                && (pointer <= (0xFFFFFFFF - size + 1) as *mut libc::c_void)
            {
                return Ok(LowMemoryBlock {
                    start: pointer as *mut u8,
                    size: size,
                });
            }
        }
        Err("Could not find mapping below 4GB")
    }
}

impl Drop for LowMemoryBlock {
    fn drop(&mut self) {
        if self.start as usize != 0 {
            unsafe {
                libc::munmap(self.start as *mut libc::c_void, self.size as libc::size_t);
            }
        }
    }
}

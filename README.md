# low_mem_alloc

## README

### What is this?

low_mem_alloc is a (very) small Rust library. Its goal is to allow the construction of slices of u8 below 4GiB on a 64bit machine.

### But why?

Some system specific code relies on old standards or even recent standards that uses u32 to specify an address. In order to write tests for this kind of code, it is useful to be able to quickly allocate a chunk of memory with a low enough address with specific data in it. low_mem_alloc does just that.

### How do I use it?

It is fairly simple. Just call `LowMemoryBlock::new(size:usize)` with a size as a parameter or directly `LowMemoryBlock::new_from_slice(data: &[u8])` with a slice to populate it. Then you can access it with `LowMemoryBlock::as_slice()` and `LowMemoryBlock::as_slice_mut()`.

use super::*;

#[test]
fn both_slices_give_same_address() {
    let mem_block = LowMemoryBlock::new(1);

    match mem_block {
        Ok(block) => {
            assert_eq!(block.as_slice().as_ptr(), block.as_slice_mut().as_ptr() as *const _);
        },
        Err(error) => {
            panic!("Could not allocate memory block with error \"{}\".", error);
        },
    }
}

#[test]
fn both_slices_give_same_size() {
    let mem_block = LowMemoryBlock::new(1);

    match mem_block {
        Ok(block) => {
            assert_eq!(block.as_slice().len(), block.as_slice_mut().len());
        },
        Err(error) => {
            panic!("Could not allocate memory block with error \"{}\".", error);
        },
    }
}

#[test]
fn correct_sizes_pass() {
    let sizes = [1usize,4,0x1000,0xF3452F,42];

    for size in sizes.iter() {
        let mem_block = LowMemoryBlock::new(*size);
        match mem_block {
            Ok(block) => {
                assert_eq!(block.as_slice().len(), *size);
            },
            Err(error) => {
                panic!("Could not allocate memory block with size {}. Error is \"{}\".", size, error);
            },
        }
    }
}

#[test]
fn fails_on_invalid_sizes() {
    let sizes = [0usize,0xFFFFFFFF,0x100000000,0x123456789,0x987654321,0xFFFFFFFFFFFFFFFF];

    for size in sizes.iter() {
        let mem_block = LowMemoryBlock::new(*size);
        match mem_block {
            Ok(_) => {
                panic!("Should not have allocated memory block with size {}.", size);
            },
            Err(_) => (),
        }
    }
}

#[test]
fn starts_in_low_mem() {
    let sizes = [1usize,4,0x1000,0xF3452F,42];

    for size in sizes.iter() {
        let mem_block = LowMemoryBlock::new(*size);
        match mem_block {
            Ok(block) => {
                assert!((block.as_slice().as_ptr() as usize) < 0x100000000);
            },
            Err(error) => {
                panic!("Could not allocate memory block with size {}. Error is \"{}\".", size, error);
            },
        }
    }
}

#[test]
fn ends_in_low_mem() {
    let sizes = [1usize,4,0x1000,0xF3452F,42];

    for size in sizes.iter() {
        let mem_block = LowMemoryBlock::new(*size);
        match mem_block {
            Ok(block) => {
                assert!((block.as_slice().as_ptr() as usize + size-1) < 0x100000000);
            },
            Err(error) => {
                panic!("Could not allocate memory block with size {}. Error is \"{}\".", size, error);
            },
        }
    }
}

#[test]
fn new_from_slice_test() {
    let mut data = [42u8; 152466];
    for (i,num) in data.iter_mut().enumerate() {
        if i % 2 == 1 {
            *num = 43u8;
        }
    }

    let mem_block = LowMemoryBlock::new_from_slice(&data[..]);
    if let Err(error) = mem_block {
        panic!("Could not allocate memory block with size {}. Error is \"{}\".", data.len(), error);
    }

    assert_eq!(&data[..], mem_block.unwrap().as_slice());
}

fn data_transfer(data: &[u8]) {
    let mem_block = LowMemoryBlock::new(data.len());

    if let Err(error) = mem_block {
        panic!("Could not allocate memory block with size {}. Error is \"{}\".", data.len(), error);
    }

    let block = mem_block.unwrap();
    block.as_slice_mut().copy_from_slice(data);

    assert_eq!(data, block.as_slice());
}

#[test]
fn data_transfer1() {
    let data1 = [42u8; 5642];
    data_transfer(&data1[..]);
}

#[test]
fn data_transfer2() {
    let data2 = [42u8, 53, 64, 75, 86, 97, 31, 20];
    data_transfer(&data2[..]);
}

#[test]
fn data_transfer3() {
    let mut data3 = [42u8; 152466];
    for (i,num) in data3.iter_mut().enumerate() {
        if i % 2 == 1 {
            *num = 43u8;
        }
    }

    data_transfer(&data3[..]);
}

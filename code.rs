use std::ptr;
#[inline(never)]
pub fn print_var(v: u8) {
    println!("{v}");
}
pub unsafe fn fn12_rs() {
    let mut bool_storage: bool = false;
    let mut v9: usize = 0;

    'l0: loop {
        let mut v20 = [197_u8; 8];
        let v20_ptr = ptr::addr_of_mut!(v20);
        let mut v12: *mut u8 = core::ptr::addr_of_mut!((*v20_ptr)[v9]);
        v9 = 2_usize; // unused but necessary write
        loop { // only runs once, but necessary
            match *v12 {
                197 => {
                    let mut match_condition: u64 = 0;
                    let mut v33: *mut bool = core::ptr::addr_of_mut!(bool_storage);
                    let mut key_read: (bool, u8) = (false, 0);
                    let mut v39: (usize, [u32; 6]) = (0, [0; 6]);

                    // Taken
                    'l2: loop {
                        (*v20_ptr) = [11_u8; 8]; // What LLVM with low mir-opt prints
                        (*v12) = 22; // What Miri prints
                        loop {
                            let v21 = *v20_ptr;
                            match match_condition {
                                0 => {
                                    // Taken
                                    v39.1 = [1; 6];
                                    match_condition = 2;
                                    v39.0 = 6;
                                    let v17 = v33;
                                    v33 = core::ptr::addr_of_mut!(key_read.0);
                                    key_read.1 = *v12;
                                    (*v17) = true;
                                    (*v20_ptr) = v21;
                                    match v39.0 {
                                        6 => {
                                            // Taken
                                            print_var(key_read.1);
                                        }
                                        0 => continue 'l2,
                                        _ => return,
                                    }
                                }
                                2 => return,
                                _ => continue 'l0,
                            }
                        }
                    }
                }
                _ => {
                    // Dead code but necessary
                    v12 = core::ptr::addr_of_mut!((*v20_ptr)[2]);
                }
            }
        }
    }
}
pub fn main() {
    unsafe {
        fn12_rs();
    }
}

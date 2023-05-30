use std::ptr;
pub fn print_var(v: u8) {
    println!("{v}");
}
pub unsafe fn fn12_rs() {
    let mut v2: bool = false;
    let mut v8: u64;
    let mut v9: usize = 0;
    let mut v17: *mut bool;
    let mut v21: [u8; 8];
    let mut v31: (bool, u8, usize, f32) = Default::default();
    let mut v33: ([u128; 7], *mut i8, *mut bool) = ([0; 7], ptr::null_mut(), ptr::null_mut());
    
    let mut v39_0: usize = 0;
    let mut v39_2: ([u32; 6], usize, *mut [u32; 6]) = ([0; 6], 0, ptr::null_mut());

    let ret: *mut bool = core::ptr::addr_of_mut!(v2);
    'l0: loop {
        let mut v20 = [197_u8; 8];
        let v20_ptr = ptr::addr_of_mut!(v20);
        let mut v12 = core::ptr::addr_of_mut!((*v20_ptr)[v9]);
        v9 = 2_usize;
        loop {
            match *v12 {
                197 => {
                    // Taken
                    v8 = 13978819448286864680_u64;
                    v33.2 = ret;
                    match v39_0 {
                        0 => {
                            // Taken
                            'l2: loop {
                                (*v20_ptr) = [11_u8; 8]; // What LLVM with low mir-opt prints
                                (*v12) = 22; // What Miri prints
                                loop {
                                    v21 = *v20_ptr;
                                    match v8 {
                                        13978819448286864680 => {
                                            // Taken
                                            v39_2 .0 = [2262110980_u32; 6];
                                            v8 = 2;
                                            v39_0 = 6;
                                            v17 = v33.2;
                                            v33.2 = core::ptr::addr_of_mut!(v31.0);
                                            v31.1 = *v12;
                                            (*v17) = true;
                                            (*v20_ptr) = v21;
                                            match v39_0 {
                                                6 => {
                                                    // Taken
                                                    print_var(v31.1);
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
                        _ => return,
                    }
                }
                4 => {
                    v12 = core::ptr::addr_of_mut!((*v20_ptr)[v9]);
                }
                _ => return,
            }
        }
    }
}
pub fn main() {
    unsafe {
        fn12_rs();
    }
}
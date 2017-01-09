#![allow(unused_imports, unused_variables, non_snake_case, dead_code)]
use ffi;
use std::fmt;
use std::ptr;
use libc::{c_int, c_char, c_uchar};
use bn::{BigNum, BigNumRef, MSB_MAYBE_ZERO};

pub struct Verifier {
    salt: Vec<u8>,
    verifier: Vec<u8>
}

pub fn create_verifier() {
    // TODO
}

#[cfg(test)]
mod tests {
    use ffi::{self, BIGNUM};
    use std::ffi::{CStr, CString};
    use std::ptr;

    use types::OpenSslTypeRef;
    use bn::{BigNum, BigNumRef, MSB_MAYBE_ZERO};

    #[test]
    fn get_default_gN() {
        unsafe {
            let gn_4096 = CString::new("4096").unwrap();
            let default_gn = ffi::SRP_get_default_gN(gn_4096.as_ptr()).as_ref();
            if let Some(gn) = default_gn {
                println!("gN Structure: {:?}", gn);
                let gn_id = CStr::from_ptr(gn.id);
                println!("gN ID: {:?}", gn_id);
                let gn_g = gn.g;

                // generate a random salt
                let mut salt = BigNum::new().unwrap();
                salt.rand(256, MSB_MAYBE_ZERO, true).unwrap();

                //println!("{:?}", salt);

                // salt as a byte-array
                //let salt_ba = salt.to_vec();
                //println!("{:?}, {}", salt_ba, salt_ba.len());


                // create a verifier from a random salt
                let user = CString::new("user").unwrap();
                let pass = CString::new("pass").unwrap();
                let verifier = ptr::null_mut();
                let salt_ptr = salt.as_ptr();
                let res = ffi::SRP_create_verifier_BN(user.as_ptr(),
                                                 pass.as_ptr(),
                                                 salt.as_ptr(),
                                                 verifier,
                                                 gn.N,
                                                 gn.g);
                println!("{}", res);
                println!("{:p}", verifier);
                //let verifier_ba = BigNumRef::from_ptr(verifier as *mut _);

            }
        }
    }
}

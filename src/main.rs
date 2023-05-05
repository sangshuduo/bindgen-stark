use std::{ffi::c_void, fmt::Debug};

#[repr(C, align(8))]
struct BoundaryConstraints {
    data: [u8; 48],
}
impl Debug for BoundaryConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoundaryConstraints")
            .field("data", &"...")
            .finish()
    }
}

#[repr(C, align(8))]
#[repr(C, align(8))]
#[derive(Debug)]
struct CppVector {
    begin: *mut c_void,
    end: *mut c_void,
    end_cap: *mut c_void,
}
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
struct BairInstance {
    vectors_len: usize,
    domain_size_indicator: u16,
    constraints_assignment: *mut c_void,
    constraints_permutation: *mut c_void,
    boundary_constraints: BoundaryConstraints,
    padding_pi: CppVector,
}
#[repr(C)]
#[derive(Debug)]
struct BairWitness {
    assignments: *mut c_void,
    permutation: *mut c_void,
}
#[repr(C)]
#[derive(Debug)]
pub struct Bair {
    instance: BairInstance,
    witness: BairWitness,
}

extern "C" {
    pub fn wrap_bair() -> Bair;
}

fn main() {
    println!("Hello, world!");
    unsafe {
        let bair = wrap_bair();
        dbg!(&bair);
        dbg!(std::mem::size_of_val(&bair.instance));
        dbg!(std::mem::size_of_val(&bair.witness));
    }
}

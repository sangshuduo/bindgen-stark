#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    use self::super::root;
    pub mod std {
        #[allow(unused_imports)]
        use self::super::super::root;
        use std::os::raw::c_void;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct allocator_traits {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct __compressed_pair {
            pub _address: *mut c_void,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct vector {
            pub __begin_: root::std::vector_pointer,
            pub __end_: root::std::vector_pointer,
            pub __end_cap_: root::std::__compressed_pair,
        }
        pub type vector_pointer = root::std::vector___alloc_traits;
        pub struct pair<_T1, _T2> {
            pub first: _T1,
            pub second: _T2,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T1>>,
            pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<_T2>>,
        }
        pub type vector___alloc_traits = root::std::allocator_traits;
        pub type qvector_pointer = root::std::vector___alloc_traits;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct __make_tree_node_types {
            pub _address: u8,
        }
        pub type __tree__NodeTypes = root::std::__make_tree_node_types;
        pub type __tree_key_type = root::std::__tree__NodeTypes;
        pub type __tree___iter_pointer = root::std::__tree__NodeTypes;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __tree {
            pub __begin_node_: root::std::__tree___iter_pointer,
            pub __pair1_: root::std::__compressed_pair,
            pub __pair3_: root::std::__compressed_pair,
        }
        pub type map___base = root::std::__tree;
        #[repr(C)]
        #[derive(Debug)]
        pub struct map {
            pub __tree_: root::std::map___base,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct unique_ptr {
            pub __ptr_: root::std::__compressed_pair,
        }
    }
    pub mod libstark {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug)]
        pub struct Sequence__bindgen_vtable(::std::os::raw::c_void);
        pub struct Sequence {
            pub vtable_: *const Sequence__bindgen_vtable,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct BairWitness {
            pub assignment_: root::libstark::BairWitness_assignment_ptr,
            pub permutation_: root::libstark::BairWitness_permutation_ptr,
        }
        pub type BairWitness_color_t = root::std::vector;
        pub type BairWitness_assignment_t = root::libstark::Sequence;
        pub type BairWitness_assignment_ptr = root::std::unique_ptr;
        pub type BairWitness_permutation_t = root::libstark::Sequence;
        pub type BairWitness_permutation_ptr = root::std::unique_ptr;

        #[repr(C)]
        #[derive(Debug)]
        pub struct BairInstance {
            pub vectorsLen_: usize,
            pub domainSizeIndicator_: ::std::os::raw::c_short,
            pub constraintsAssignment_: root::libstark::BairInstance_constraintsPtr_t,
            pub constraintsPermutation_: root::libstark::BairInstance_constraintsPtr_t,
            pub boundaryConstraints_: root::libstark::BairInstance_boundaryConstraints_t,
            pub paddingPi_: root::std::vector,
        }
        pub type BairInstance_constraintsPtr_t = root::std::unique_ptr;
        pub type BairInstance_point_t = root::std::pair<usize, usize>;
        pub type BairInstance_boundaryConstraints_t = root::std::map;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Bair {
    pub instance: root::libstark::BairInstance,
    pub witness: root::libstark::BairWitness,
}

#[link(name = "wrapper")]
extern "C" {
    pub fn wrap_bair() -> *mut Bair;
}

fn main() {
    println!("Hello, world!");
    unsafe {
        let ptr = wrap_bair();
        dbg!(&*ptr);
    }
}

include!("../bindings.rs");

#[link(name = "BairWitnessChecker_UTEST")]
extern "C" {
    fn generate_valid_constraints();
}

fn main() {
    println!("Hello, world!");
    unsafe {
        generate_valid_constraints();
    }
}

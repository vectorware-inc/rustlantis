mod fuzz;
mod reduce;
fn main() {
    fuzz::run().unwrap();
}

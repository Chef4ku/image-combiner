// Get input and return it
fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    pub img1: String,
    pub img2: String,
    pub output: String,
}

// puts the inputs in the strings
impl Args {
    pub fn new() -> Self {
        Args {
            img1: get_nth_arg(1),
            img2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}

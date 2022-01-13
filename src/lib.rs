pub trait HelloMacro {
    fn hello_macro();
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        // example of a procedural macro
        println!("Hello, Macro! My name is Pancakes!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), crate::Pancakes::hello_macro());
    }
}

pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Pancakes;

    impl HelloMacro for Pancakes {
        fn hello_macro() {
            // example of a procedural macro
            println!("Hello, Macro! My name is Pancakes!");
        }
    }

    #[test]
    fn it_works() {
        assert_eq!((), Pancakes::hello_macro());
    }
}

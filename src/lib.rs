pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod main;

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn this_is_cool_stuff(){
    //     println!("Hello, world!");
    //     assert_eq!(1 + 1, 2)
    // }

    #[test]
    fn run_main_function(){
        main::main();
    }
}

#[cfg(test)]
mod tests {
    /*
     * `panic!` will print the error message, unwind the stack and finally exit
     * the program.
     * Note: By `unwind`, meaning pop up the call stack step by step.
     * In multithread programs it will exit the thread in which the panic!
     * occurs, not the whole program.
     */

    // call marco `panic!` to interrupt a thread
    // annotate the test function with #[should_panic] to validate expected panic
    // validate panic contains substring with argument `expected`
    #[test]
    #[should_panic(expected = "should panic!")]
    fn panic_and_catch() {
        panic!("this test should panic!");
    }
}

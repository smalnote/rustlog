use std::future::ready;

fn main() {
    let mut vec = vec![];

    // async block: `|| async {}`
    // cannot capture outer variables
    let _async_block = || async {
        // captured variable cannot escape `FnMut` closure body
        // vec.push(ready(String::from("Hello")).await);
        println!("{:?}", vec);
    };

    // async closure: `async || {}`
    let mut closure = async || {
        vec.push(ready(String::from("Hello")).await);
    };

    async_std::task::block_on(closure());
    async_std::task::block_on(closure());
    async_std::task::block_on(closure());

    println!("{:?}", vec);
}

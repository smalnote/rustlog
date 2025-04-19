#[cfg(test)]
mod tests {
    use std::collections::{LinkedList, VecDeque};

    use async_std::task;

    #[test]
    fn async_closure() {
        use std::future::ready;
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

        task::block_on(closure());
        task::block_on(closure());
        task::block_on(closure());

        println!("{:?}", vec);
    }

    #[test]
    fn asnyc_closure_trait() {
        async fn take_async_closure(mut c: impl for<'a> AsyncFnMut(&'a Vec<String>)) {
            let v = vec![];
            c(&v).await;
        }

        async fn take_async_generic<F>(_: impl for<'a> Fn(&'a Vec<String>) -> F)
        where
            F: Future<Output = ()>,
        {
        }

        let async_block = |_: &Vec<String>| async {};
        let async_closure = async |_: &Vec<String>| {};

        task::block_on(take_async_closure(async_block));
        task::block_on(take_async_closure(async_closure));

        task::block_on(take_async_generic(async_block));
        // mismatched types
        // task::block_on(take_async_generic(async_closure));
    }

    #[test]
    fn tuple_iterator() {
        let (squares, cubes, tesseracts): (Vec<_>, VecDeque<_>, LinkedList<_>) =
            (0i32..10).map(|i| (i * i, i.pow(3), i.pow(4))).collect();

        println!("{squares:?}");
        println!("{cubes:?}");
        println!("{tesseracts:?}");
    }
}

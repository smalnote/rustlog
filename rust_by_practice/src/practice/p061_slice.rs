/// Slice 是数组的常量引用，包含切片的起始地址和长度，配合类型即可静态确定。
/// &str 其实是 &[u8] 的语法糖，即合法的字节数组的切片。
/// 由于是引用，所以不能单独存在 [T], 必需是 &[T] &mut[T] Box<[T]> 等。
#[cfg(test)]
mod test {
    #[test]
    fn test_creating_slice() {
        // slice from array
        let array: [i32; 10] = [42; 10];
        let slice_first_two: &[i32] = &array[..2];
        let slice_last_five: &[i32] = &array[array.len() - 5..];

        assert_eq!(slice_last_five.len(), 5);
        assert_eq!(slice_first_two.len(), 2);

        // slice from vector
        let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let slice_first_two: &mut [i32] = &mut vector[..2];
        slice_first_two[1] = 0;
        assert_eq!(slice_first_two, &mut [1, 0]);

        let slice_last_five: &mut [i32] = &mut vector[array.len() - 5..];
        slice_last_five[4] = 0;
        assert_eq!(slice_last_five, &mut [6, 7, 8, 9, 0]);

        assert_eq!(vector, vec![1, 0, 3, 4, 5, 6, 7, 8, 9, 0]);

        // box slice: on heap
        // [1, 2, 3, 4, 5] 根据上下文可以构造为 slice 或 array
        let slice_on_heap: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);
        assert_eq!(*slice_on_heap, [1, 2, 3, 4, 5]);
        println!("slice: {slice_on_heap:?}");

        let array_on_heap: Box<[i32; 5]> = Box::new([1, 2, 3, 4, 5]);
        assert_eq!(*array_on_heap, [1, 2, 3, 4, 5]);
        println!("array: {array_on_heap:?}");
    }
}

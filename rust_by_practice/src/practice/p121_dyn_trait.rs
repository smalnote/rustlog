// dyn Trait 为特征胖指针，包含指向实例的指针外加一个具体类型 T 实现 Trait 的 vtable
// vtable 中包含:
// 0. fn drop_in_place::<T>(...)
// 1. usize size_of::<T>()
// 2. usize align_of::<T>()
// 3+.  trait 方法函数
//
// 注意：vtable 是以 Trait + T 粒度的，编译会为不同类型 Q 生成不同的 vtable
// impl Trait for T {} -> vtable_for_T_as_Trait
// impl Trait for Q {} -> vtable_for_Q_as_Trait
//
// align_of 的必要性，align 为具体类型的以字节数为单为的对齐
// 如 struct S { u8, u64 } align 为 8, 会在第一个字段后面 padding 7 byte, size 16
// struct S { u8 } align 为 1, size 1
// CPU 从内存读取对象需要对齐起始地址，起始地址为 align 整数倍, 否则可以导致性能下降或未定义行为
#[cfg(test)]
mod test {
    trait Bird {
        fn fly(&self);
    }

    struct Mocking {}
    impl Bird for Mocking {
        fn fly(&self) {
            println!("mocking bird fly")
        }
    }
    struct Cuckoo {}
    impl Bird for Cuckoo {
        fn fly(&self) {
            println!("cuckoo fly")
        }
    }
    #[test]
    fn test_dyn_trait() {
        fn do_fly(bird: &dyn Bird) {
            bird.fly();
        }

        // dyn Trait 为胖指针，因此在同一个切片中可以传入实现 Trait 的不同具体类型
        fn do_fly_all(birds: &[&dyn Bird]) {
            birds.iter().for_each(|bird| bird.fly());
        }

        let m1 = Mocking {};
        let m2 = Mocking {};
        let c1 = Cuckoo {};
        let c2 = Cuckoo {};

        do_fly(&m1);
        do_fly(&c1);

        do_fly_all(&[&m1, &m2, &c1, &c2]);
    }

    // 作为简单的对比， impl Trait 实际上是泛型的语法糖，不能在同一个切片中传入不现具体类型
    #[test]
    fn test_impl_trait() {
        fn do_fly_all(birds: &[impl Bird]) {
            birds.iter().for_each(|bird| bird.fly());
        }

        let m1 = Mocking {};
        let m2 = Mocking {};
        let c1 = Cuckoo {};
        let c2 = Cuckoo {};

        do_fly_all(&[m1, m2]);
        // error: mismatched types expected `Mocking`, found `Cuckoo`
        // do_fly_all(&[m1, c2]);

        // 等价写法
        fn do_fly_all_2<T: Bird>(birds: &[T]) {
            birds.iter().for_each(|bird| bird.fly());
        }
        do_fly_all_2(&[c1, c2]);
    }
}

use std::marker::Unsize;
use std::ops::CoerceUnsized;

struct MyStruct<T: ?Sized> {
    value: Box<T>,
}

impl<T, U> CoerceUnsized<MyStruct<U>> for MyStruct<T>
where
    T: Unsize<U>,
{
}

fn main() {
    let sized_instance: MyStruct<[i32; 3]> = MyStruct {
        value: Box::new([1, 2, 3]),
    };

    let unsized_instance: MyStruct<[i32]> = sized_instance;
    println!("First element: {}", unsized_instance.value[0]);
    println!("Length of the slice: {}", unsized_instance.value.len());
}

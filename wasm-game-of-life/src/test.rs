// turn off some lint warnings while developing
#![allow(unused_variables)]
#![allow(dead_code)]

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
enum Thing {
    Dead,
    Alive
}

fn main() {
    println!("Hello world");
    let thing = Thing::Dead;
    println!("{:?}", thing);

    let width = 64;
    let height = 64;
    println!("{}", width);
    let thing = 0..(width * height);
    // println!("{}", thing);
}

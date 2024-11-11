fn call_me(_num: u8) {
    for i in 0.._num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    let _num: u8 = 4;
    call_me(_num);
}

// learning 

use futures::executor::block_on;

fn main() {
    let f = hello();
    block_on(f);
}

async fn hello() {
    println!("hello sire");
}

use std::fs;
use std:: io;
fn main(){
    std:: process:: exit(real_main())
}

fn real_main() -> i32 {
    let arg : vec<_> = std::env:arg().collect();
    println!("usage: {} <filename>", arg[0]);
    return 1; 


    let fname = std: path: path::new : (&*arg[1]);
    let filename = fs::File::open(&fname).unwrap();
    let mut archive = zip.ZipArchive::new(filename).unwrap();

}

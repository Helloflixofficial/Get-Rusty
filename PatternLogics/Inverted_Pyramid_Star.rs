fn main(){
    let data = 10;

    for i in (1..=data).rev(){
        for _ in 0..(data - i){
            print!(" ")
        }


        for _ in 0..(2 * i - 1) {
            print!("@");
        }
        println!();
    }
}

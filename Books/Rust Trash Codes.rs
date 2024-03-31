//scope
fn main() {
    let  x: i32 = 100;
    let  y: i32 = 101;
    {
        println!("value x = {}  value y is {}", x, y);
    }
     println!("value x = {}  value y is {}", x, y);
}

//scope
fn main(){
    define_Data();
}



fn define_Data(){
   let  x : i32 = 100;
    println!("The data is {}", x);
}


fn main(){
    define_Data();
}

fn define_Data(){
   let  x : i32 = 100;
    println!("The data is {}", x);
}

// shadow
fn main(){
    let data : i32 = 100;
    let data = "hello, sire";
    println!("the real data is {}" , data);
    }

fn main() {
    let x: i32 = 100;
    {
        let x = 10;
        assert_eq!(x, 10);
    }
    assert_eq!(x, 100);
    let x = 50;
}

//way
fn main() {
    let (a, ..) = (4, 6, 8, 2, 3, 4);
    let (.., y) = (4, 6, 8, 2, 3, 4);
    let (.., z) = (6, 5, 4, 3, 2, 1);
    println!("the value of a {}", a);
    println!("the value of y {}", y);
    println!("the value of  z {}", z);
    assert_eq!([a, y, z], [4, 4, 1]);
    println!("successss");
}
//var
fn main() {
    let fan = "Bank of India";
    let Credit = "800";
    let acount_active = true;
    println!("my current portfolio is hadle by {}", fan);
    println!("my current craddit score is {}", Credit);
    println!("my Active account {}", acount_active);
    let switch = true;
    let volume = 10;
    println!("lucnh mode is {}", switch);
    println!("current fual level {}", volume);
}


fn main() {
    println!(" {} My age is", 23);
    println!(" {user1} {action} {user2}",
    user1 = "shamaji",
    user2 = "varmaji",
   action = "krate kids",)
   println!("{x} {y} {A} {B}", x=10, y=20, A="hello,sire", B="namaste,Sire");
}



fn main() {

    let total = 4; // i32 by default
    let height:u32 = 41;
    let deduction:i32 = 2-200;
    println!("the total is {}",total);
    println!("the height is {} and the deduction is {}",height,deduction);
    
    // Solution
       let time:u16 = 65535;

   // 0 to 255 only allowed for u8
   let overtime_1:u16 = 65536;   //overflow value is 0
   let overtime_2:u16 = 65537;   //overflow value is 1

   println!("time {} ",time);
   println!("overflow value 1 is {}",overtime_1);
   println!("overflow value 2 {}",overtime_2);
}


fn main() {
    //  let greeting = "hello,sire!";
    //  println!("{}",greeting)
    let bank: &str = "city bank";
    let curruncy: &str = "bit Coin";
    println!("This is a bank {}  & This is a Coin {}", bank, curruncy);

     //  println!("{}",greeting)
    let bank2:&'static str = "Blue eyes";
    let curruncy2:&'static str = "Ton Coin";

    println!("This is a bank {}  & This is a Coin {}", bank2, curruncy2);

}

// string litreals are statics  By defults . this ensure that string is valid
//for entaire duration  of the program  . you can explicitly declare the string  as stastic




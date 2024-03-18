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

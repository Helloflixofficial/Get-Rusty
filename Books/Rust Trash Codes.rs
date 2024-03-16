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

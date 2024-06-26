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


fn main() {

    let mut password = "pokimon,".to_string();
     password.push_str(" random string sire");
    println!("changeing password is {}", password)
}



////////////////////////////////////////
Relational Operators

Relational Operators check or define the relationship equavalency 
between two elements. Relational operators are used to compare two or more values. 
Relational operators return a Boolean value − true or false.

x = 5
y = 2

Show Examples

      	Operator	Description	Example
1	>	Greater than	(x > y) is true
2	<	Lesser than	(x < y) is false
3	>=	Greater than or equal to	(x >= y) is true
4	<=	Lesser than or equal to	(x <= y) is false
5	==	Equality	(x == y) is false
6	!=	Not equal	(x != y) is true
////////////////////////////////////

Logical Operators

Logical Operators are used to combine and check two or more conditions. 

Logical operators return a Boolean value. 

Show Examples

x = 1
y = 2

Sr.No	Operator	Description	Example

1)	&& (And)	The operator returns true as long as 
all the expressions specified return true	
(x > 0 && y > 3) false

2)	||(OR)	The operator returns true if at least one of the expressions specified 
return true	(x > 0 || y > 3) true

3)	! (NOT)	The operator returns the inverse of the expression’s result. For E.g.: !(>5) 
returns false	!(x < 4 ) false

fn main() {
    let user = "todd";
    if user.len() == 4 {
        println!("pass");
    } else {
        println!("faild")
    }
}

fn main() {
    let user = "todd";
    if user.len() == 4 {
        println!("pass");
    } else {
        println!("faild")
    }
    if user.len() == 3 {
        println!("pass");
    } else {
        println!("faild")
    }
}

fn main() {
    let password = "sharnaji";
    if password.len() > 4 {
        println!("Thanks For Your Support & fo password");
    } else if password.len() > 2 {
        println!("Atleast have more then 3 words");
    } else {
        println!("password is to short")
    }
}


fn main() {
    let micro = "xyz";
    let body = match micro {
        "xyz" => {
            println!("Found match for microbine");
            "Tummy bio"
        },
        "html" => "hiper language",
        "Time" => "temporary",
        "robot" => "unknown data",
        _ => "No match found"
    };
    println!("The bio match {}", body);
}


fn main(){
    data = "kktk";
    meta = match data {
     "a" =>  "b"
     "c" =>  "d"
     "p" => "l"
     "kktk" => "party"
    }
}


fn main(){

    // for loop example  - we call this a definite loop because we run it until
    
   for a in 1..20{ // 20 is not inclusive
      if a==2 {
      
      // The continue statement skips the subsequent statements 
      // in the current iteration and takes the control back to the beginning of the loop
         continue;
      }
      println!("a is {}",a);
   }
   
   // an indefinite loop is used when the number of iterations in a loop is indeterminate or unknown
   
   let mut b = 0;
   while b < 5{
      b = b + 1;
      println!("loop b value is {}",b);
   }
   
   //  also indefinite example
   
     //while true

   let mut c = 0;
   loop {
      c-=1;
      println!("c={}",c);

      if c==-10 {
      // breaks ends the loop and moves control to the next command outside the loop.
         break;
      }
   }
   
   
   // Exercise Solution
   
    let mut count: u32 = 0;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("Welcome to miami!");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Time to call it a dat!");

            // Exit this loop
            break;   
        }
    }
   
}

fn main() { 
   let tuple:(i8,f32,i32) = (2, 2.333, 22);
   println!("{:?}",tuple);
   println!("first value is :{:?}",tuple.0);
   
   // exercise solution
   let user2:(i32,bool,&str) = (30,true,"Jack");
   user_data(user2);
}

// string litreals are statics  By defults . this ensure that string is valid
//for entaire duration  of the program  . you can explicitly declare the string  as stastic


// syntexx for function  === synt code to remember

// 1 returning Function
// 2 parameterized Function 

//some function can have return statements that returns value back to the caller
//some function can have return statements that returns value back to the caller
fn main() {
    fn_main1();
    fn_main();
    another_function(100*5-499);

    let y = plus_one(5);
    println!("the value of y : {}", y)
     let H = data_base();
}


fn fn_main1() -> bool {
    return true;
}

fn fn_main() {
    println!("hello im a fuction");
}

// another function

fn another_function(x: i32) {
    println!("the value of x : {}", x)
}

fn plus_one(z:i32) -> i32{
    return z + 1 ;
}


fn main(){
    let tuple:(i8,f32,i32) = (2,5.6,500);
    println!("{:?}",tuple);
    println!("the sec value is {:?}",tuple.1);
    println!("the first value is {:?}",tuple.0);
    println!("the third value is {:?}",tuple.2);
}


fn main(){
  let user2:(i32,bool,&str) = (30,true,"sharmaji");  
  user_data(user2); 
}

fn user_data(x:(i32,bool,&str)){
   
    let (age,active,name) = x;
    println!("age : {} , active : {} , name: {}",age,active,name);
}


fn main(){
 let data:(i32,&str,bool) = (500,"sharmaji",false);
 userdata(data);
}


fn userdata(x:(i32,&str,bool)){
  let (date,name,active) = x;
  println!("date:{}, name : {}, active: {}",date,name,active);
}

fn main(){
 let data:(i32,&str,bool) = (500,"sharmaji",false);
 userdata(data);
 let data:(i32,&str,bool) = (1000,"boby",true);
 userdata(data);
}


fn userdata(x:(i32,&str,bool)){
  let (date,name,active) = x;
  println!("date:{}, name : {}, active: {}",date,name,active);
}


fn main() {
    let arr: [&str; 4] = ["herry", "boby", "cartoon", "mac"];
    let mut arrs = ["tommy", "thakur", "dev", "cartoonm"];
    arrs[2] = "raja";
    println!("there are the name's of heros:{:?} , {:?} ", arrs, arr);
    println!("this are the names : {:?}", arrs.len());
for value in arrs.iter(){
    println!("the value of string is : {:?}",value);
};
}


fn main(){
  let mut array:[i32;6] = [12,2,3,2,4,5];
  for i in 0..6 {
    if array[i] == 2 {
      array[i]  = 0; 
    }
    println!("THE INDEX VALUE :{}, & the actuall value : {}",i,array[i]);
  }
}


fn main(){
   let vector1 = vec![1,2,3]; 
   // vector v owns the object in heap

   //only a single variable owns the heap memory at any given time
   let vector2 = vector1; 
   //two pointers to the same content is not allowed in rust

   //Rust will check for memory access which is a key selling point for using the language


fn main(){
   let vector = vec![1,2,3];
   //display(vector);
   display(&vector);
   println!("{}",vector[1]); // this will not work
   
   
   // Exercise Solution 
   
    let mut car:String = String::from("Ferrari");
   display2(&mut car); 
   //pass a mutable reference of name
   println!("The car has been updated to: {}",car);
   
}


//fn display(x:Vec<i32>){
fn display(x:&Vec<i32>){
   println!("{:?}",x);
   
   
}

fn display2(_car:&mut String){
   println!("_car value is :{}",_car);
   _car.push_str(" F8 Tributo"); 
   //Modify the actual string,name
}    


    fn main(){
   let vector = vec![1,2,3];
   //display(vector);
   display(&vector);
   println!("{}",vector[1]); // this will not work
   
   
   // Exercise Solution 
   
    let mut car:String = String::from("Ferrari");
   display2(&mut car); 
   //pass a mutable reference of name
   println!("The car has been updated to: {}",car);
   
}



//fn display(x:Vec<i32>){
fn display(x:&Vec<i32>){
   println!("{:?}",x);
   
   
}

fn display2(_car:&mut String){
   println!("_car value is :{}",_car);
   _car.push_str(" F8 Tributo"); 
   //Modify the actual string,name
}    


    fn main(){
   let vector = vec![1,2,3];
   //display(vector);
   display(&vector);
   println!("{}",vector[1]); // this will not work
   
   
   // Exercise Solution 
   
    let mut car:String = String::from("Ferrari");
   display2(&mut car); 
   //pass a mutable reference of name
   println!("The car has been updated to: {}",car);
   
}

// Slice Example 

fn main() {
   let game = "Mario Brothers".to_string();
   println!("length of the game is {}",game.len());
   let slice = &game[0..5]; 
   println!("{}",slice);
   
   // Exercise Solutio n
    let mut nums = [1,2,3,4,5];
   slice_and_dice(&mut nums[1..3]);
   // passes references 
   println!("{:?}",nums);
}


    fn main() {
   let game = "Mario Brothers".to_string();
   println!("length of the game is {}",game.len());
   let slice = &game[0..5]; 
   println!("{}",slice);
   
   // Exercise Solutio n
    let mut nums = [1,2,3,4,5];
   slice_and_dice(&mut nums[1..3]);
   // passes references 
   println!("{:?}",nums);
}

fn slice_and_dice(slice:&mut [i32]) {
   println!("length of slice is {:?}",slice.len());
   println!("{:?}",slice);
   slice[0] = 200; 
}

    fn main() \{\
    \
    let home1 = House \{\
        door:String::from("Blue"),\
        garden:String::from("Beautiful"), \
        property_value:1000000\
    \}; \
    \
    println!("door is: \{\}, garden is: \{\}, property value is: \{\}",\
    home1.door, home1.garden, home1.property_value);\
    \
    let new_triangle = Triangle \{\
        base:10,\
        height:30 \
    };

        
fn main() {
    
    let home1 = House {
        door:String::from("Blue"),
        garden:String::from("Beautiful"), 
        property_value:1000000
    }; 
    
    println!("door is: {}, garden is: {}, property value is: {}",
    home1.door, home1.garden, home1.property_value);
    
    let new_triangle = Triangle {
        base:10,
        height:30 
    };
    
    println!("the area of the triangle is: {}", new_triangle.find_area());
}


        struct House {
    door: String,
    garden: String,
    property: u32,
}

fn main() {
    let home = House {
        door: String::from("Blue"),
        garden: String::from("beautiful"),
        property: 500_000_000,
    };
    println!("Door: {}, Garden: {}, Property: {}", home.door, home.garden, home.property);
}

        fn main() {
    
    let c1 = City {
        name:String::from("Alaska"),
        temperature:TemperatureGrade::Cold
    };
    
    let c2 = City {
        name:String::from("Miami"),
        temperature:TemperatureGrade::Hot
    };
    
    println!("{:?}",c1);
    println!("{:?}",c2);
    
    // Exercise Solution
    print_shoes(Shoes::Nike);
    print_shoes(Shoes::Loafer);
    print_shoes(Shoes::Vans);
    
}



        
pub mod songs {
    pub fn play(name:String) {
        println!("track selection: {}", name);
    } 
}

use songs::play;

// Exercise Solution

pub mod tracks {
    pub mod rock {
        pub mod indie {
            pub fn select(name:String) {
                println!("Track selection is: {}", name);
            }
        }
    }
}

use tracks::rock::indie::select;
// imported a public module

fn main() {
//    play("Kissed By A Rose".to_string());

select("Serenade".to_string());
select("French Baguette".to_string());
select("Pineapple Blues".to_string());

}


            println!("{} years old", 36);
    
    // named arguments also work
    
    println!("{user1}{action}{user2}",
            user1="Mary Lewis",
            user2="Jon Wick",
            action=" karate kicked ");
            
    
// Exercise: Within the function main add a println macro which takes two arguments, 
//x and y and stringifies them where x equals "hello " and 
// y equals "my friend".

    println!("{x}, {y}",
    x="hello",
    y="my friend")
            
}


    pub mod tracks {
    pub mod rock {
        pub mod inde {
            pub fn select(name : String){
                println!("Tracks selection is {}",name);
            }
        }
    }
}

use tracks::rock::inde::select;

fn main(){
select("Arjit Singh".to_string());
select("Ankit Tiwari".to_string());
select("Sonu Nigam".to_string());
}



        







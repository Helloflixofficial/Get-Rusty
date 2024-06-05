
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

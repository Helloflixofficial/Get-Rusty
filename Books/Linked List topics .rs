#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

fn to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cur = None;
    for &Value in vector.iter().rev() {
        let mut new_node = ListNode::new(Value);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }
    cur
}

fn main() {
    let vector = vec![0, 1, 2, 3, 4];
    println!("{:?}", to_list(vector));
}






impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_node = None;
        let mut original_node = head;

        while let Some(mut node) = original_node {
            original_node = node.next; // advancing the current node
            node.next = new_node;
            new_node = Some(node);
        }
        return new_node;
    }
}




use crate::ListNode; 
pub struct Solution;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        let mut flip_slow = false;

        while let Some(node) = fast {
            fast = &node.next;
            if flip_slow {
                slow = &slow.as_ref().unwrap().next;
            }
            flip_slow = !flip_slow;
        }

        slow.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        let middle = Solution::middle_node(list).unwrap().into_array();
        assert_eq!(middle, vec![3, 4, 5]);
    }

    #[test]
    fn ex2() {
        let list = ListNode::from_vec(&vec![1, 2, 3, 4, 5, 6]);
        let middle = Solution::middle_node(list).unwrap().into_array();
        assert_eq!(middle, vec![4, 5, 6]);
    }
}




#![allow(unused)]
struct Boeing {
    required_crew: u8,
    range: u16,
}
struct Airbus {
    required_crew: u8,
    range: u16,
}
trait Flight {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let boeing = Boeing {
        required_crew: 4,
        range: 7370,
    };
    let airbus = Airbus {
        required_crew: 7,
        range: 5280,
    };

    let boeing_is_legal = boeing.is_legal(boeing.required_crew, 18, boeing.range, 2385);

    let airbus_is_legal = airbus.is_legal(airbus.required_crew, 3, airbus.range, 2200);
    println!(
        "is the Boeing flight legal ? {} \n is the Airbus flight is legal? {}",
        boeing_is_legal, airbus_is_legal
    );
}




#![allow(unused)]
struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64,
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

fn main() {
    const EARTH_REDIUS_IN_KILOMETEERS: f64 = 6371.0;

    let mut data = Waypoint {
        name: "sharmaji".to_string(),
        latitude: 41.4075,
        longitude: -81.851111,
    };
}

fn main() {
    let mut flights: Vec<&str> = Vec::new();
    flights.push("DA113\t to Boston department at 06:20");
    flights.push("DA98\t to Boston department at 09:43");
    flights.push("DA428\t to Boston department at 12:05");
    flights.push("DA41\t to Boston department at 15:30");
    flights.push("DA2815\t to Boston department at 17:11");

    let third = flights[2];
    println!("\n {}", third);

    flights.remove(0);
    flights.remove(1);
    flights.remove(2);

    let fourth = flights.get(3);
    match fourth {
        Some(flight) => {
            println!("with actuall data{}", flight)
        }
        _ => {}
    }

    if let Some(flight_value) = flights.get(3) {
        println!("{}", flight_value);
    }

    for flights in flights.iter() {
        println!("{}", flights);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_push_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_left(1);
        list.push_left(2);
        list.push_left(3);
        list.push_left(4);
        assert_eq!(list.collect(), vec![4,3,2,1]);
    }
}



#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        let mut carry = 0;
        
        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = carry 
                + l1.as_ref().map_or(0, |n| n.val) 
                + l2.as_ref().map_or(0, |n| n.val);
            
            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
            
            if let Some(node) = l1 { l1 = node.next; }
            if let Some(node) = l2 { l2 = node.next; }
        }
        
        dummy.next
    }
}

fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    nums.into_iter().rev().fold(None, |acc, val| {
        Some(Box::new(ListNode { val, next: acc }))
    })
}

fn main() {
    // Test: [2,4,3] + [5,6,4] = [7,0,8]
    let l1 = create_list(vec![2, 4, 3]);
    let l2 = create_list(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    
    
    let mut curr = result;
    while let Some(node) = curr {
        print!("{} ", node.val);
        curr = node.next;
    }
    println!(); // Output: 7 0 8
}

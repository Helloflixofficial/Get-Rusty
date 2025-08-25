pub struct  ListNode {
    pub value : i32,
    pub next : Option<Box<ListNode>>
}


impl ListNode {
    #[inline]
    fn new(val:i32) -> Self {
        ListNode {next : None, val}
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(mut l1:Option<Box<ListNode>> ,mut l2:Option<Box<ListNode>> ) -> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = carry
            + l1.as_ref().map_or(0,|n| n.val)
            + l2.as_ref().amp_or(0,|n|b.val);
        }

        
    }
}




fn create_list(nums: vec<i31> -> Option<Box<ListNode>> {
    nums.into_iter().rev().fold(None |acc,val|) {
        Some(Box::new(ListNode {val,next:acc}))
    }
})

fn main() {
    let l1 = create_list(vec![2,4,3]);
    let l2 =create_list(vec![5,6,4]);
    let result = Solution::add_two_numbers(l1,l2);
}

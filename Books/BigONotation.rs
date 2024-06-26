fn data(n:i32) -> i32{
    let mut sum = 0;
    for i in  0..=n{
        sum += i;
    }
    sum
}
fn data2(n: i32) -> i32 {
    n * (n + 1) / 2
}
fn main(){
    let result = data(10);
    let result2 = data2(10);
    println!("the sum is {:#}",result);
    println!("the sum is {:#}",result2);
}




# Array
nums = [1, 2, 3]
nums.append(4)    # push to end
nums.pop()        # pop from end
nums[0]           # lookup
nums[1]
nums[2]


# HashMap / Set
hashMap = {}
hashMap["key"] = 10     # insert
print("key" in hashMap) # lookup
print(hashMap["key"])   # lookup
hashMap.pop("key")      # remove





nums = [1, 2, 3]
sum(nums)           # sum of array
for n in nums:      # looping
    print(n)

nums.insert(1, 100) # insert middle
nums.remove(100)    # remove middle
print(100 in nums)  # search

import heapq
heapq.heapify(nums) # build heap

# sometimes even nested loops can be O(n)
# (e.g. monotonic stack or sliding window)




fn main() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![4, 5];
    for &num1 in &nums1 {
        for &num2 in &nums2 {
            println!("{} {}", num1, num2);
        }
    }
}

fn main(){
    let num1 = vec![1,2,3];
    let num2 = vec![4,5,6];
    for  &nums1 in &num1{
        for &nums2 in &num2{
            println!("{} {}",nums1,nums2);
        }
    } 
}



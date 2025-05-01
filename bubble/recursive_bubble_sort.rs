fn recursive_bubble_sort(arr: &mut [i32], n: usize) {
    if n == 1 {
        return;
    }

    for i in 0..n - 1 {
        if arr[i] > arr[i + 1] {
            arr.swap(i, i + 1);
        }
    }

    recursive_bubble_sort(arr, n - 1);
}

fn main() {
    let mut nums = vec![5, 3, 8, 4, 2];
    println!("Before sorting: {:?}", nums);

    let len = nums.len();
    recursive_bubble_sort(&mut nums, len);

    println!("After sorting:  {:?}", nums);
}

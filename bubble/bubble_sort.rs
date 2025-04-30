fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();  // Seven as we know;
    for i in 0..len {  // 0 to 6;
        for j in 0..len - 1 - i {
            //  println!("j data : {}",j);
            if arr[j] > arr[j + 1] {

                arr.swap(j, j + 1);

            }
        }
    }
}

fn main() {
    let mut nums = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before sorting array data: {:?}", nums);
    bubble_sort(&mut nums);
    println!("After sorting array data:  {:?}", nums);
}

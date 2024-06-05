fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        return 0.0;
    }
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("The median is {}", median(&arr));
}

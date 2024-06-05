fn max_sub_array_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];
    for &num in &nums[1..] {
        current_sum = num.max(current_sum + num);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    let nums = [4, -1, 2, 1, -5, 4];
    println!("The maximum subarray sum is {}", max_sub_array_sum(&nums));
}

fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in nums.iter() {
        match sum.checked_add(num) {
            Some(new_sum) => sum = new_sum,
            None => return None,
        }
    }
    Some(sum)
}
fn main() {
    let nums = &[1, 2, 3, 4, 5];
    match sum_u32(nums) {
        Some(sum) => println!("整数集合的和为：{}", sum),
        None => println!("求和失败：发生了溢出！"),
    }
}

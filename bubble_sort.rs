fn bubble_sort<T: PartialOrd + Clone>(nums: Vec<T>) -> Vec<T> {
    let mut items = nums.clone();
	let len = items.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if items[j] > items[j + 1] {
                // swap elements
                items.swap(j, j + 1);
            }
        }
    }
    items
}
fn main() {
 
    let  list = vec![1, 34, 50, 200, 34, 51, 25, 100, 65];
    let sorted_list = bubble_sort(list.clone());
    println!("{:?}  ", sorted_list);
 
    let  list = vec!['D', 'e', 'A', 'C', 'a', 'W'];
    let sorteed_list = bubble_sort(list.clone());
    println!("{:?}  ", sorteed_list);
}

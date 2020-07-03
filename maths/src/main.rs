use std::collections::HashMap;

fn main() {
    let nice = vec![2, 2, 3, 3, 7];
    let (mean, mode, median) = maths_values(&nice);
    println!("mean:{} mode:{} median:{}", mean, mode, median);
}

fn maths_values(nums: &Vec<i32>) -> (f64, i32, f64) {
    let mut sum: i32 = 0;
    let len = nums.len();
    let mut all_nums: HashMap<i32, i32> = HashMap::new();
    for num in nums.iter() {
        // dereference pointer to access the actual value
        sum += *num;
        let a = all_nums.entry(*num).or_insert(0);
        // dereference, increase by 1
        *a += 1;
    }
    let mut mode = 0;
    let mut often = 0;
    for (k, v) in all_nums {
        if v > often {
            often = k;
            mode = k;
        }
    }
    let mut new_vec = nums.clone();
    new_vec.sort();
    let mean = sum as f64 / len as f64;
    let newLen = new_vec.len();
    let mut median: f64 = 0f64;
    if newLen % 2 == 1 {
        median = new_vec[newLen / 2] as f64;
    } else {
        let first = new_vec[newLen / 2 - 1];
        let second = new_vec[newLen / 2];
        median = (first as f64 + second as f64) / 2f64;
    }
    return (mean, mode, median);
}

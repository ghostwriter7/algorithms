
fn main() {
    let test: Vec<i32> = vec![1, 9, 81, 122, 254, 331, 596, 1233, 9002, 11000, 952322];

    let has_331 = binary_search(&test, 331);
    let has_9001 = binary_search(&test, 9001);

    println!("Test array has 331: {has_331}");
    println!("Test array has 9001: {has_9001}");
}

fn binary_search(haystack: &Vec<i32>, needle: i32) -> bool {
    let mut lo  = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let m = ((lo + (hi - lo) / 2) as f64).floor() as usize;
        let v = haystack[m];

        if v == needle {
            return true;
        } else if v < needle {
            lo = m + 1;
        } else {
            hi = m;
        }
    }

    return false;
}
const TEST_ARRAY: [i32; 10] = [1, 2, 3, 6, 9, 11, 69, 222, 11213, 12332];

fn linear_search(haystack: [i32; 10], needle: i32) -> bool {
    for element in haystack {
        if element == needle {
            return true;
        }
    }

    return false;
}

fn main() {
    let has_eleven = linear_search(TEST_ARRAY, 11);
    let has_seventy = linear_search(TEST_ARRAY, 70);

    println!("TEST_ARRAY has eleven: {has_eleven}");
    println!("TEST_ARRAY has seventy: {has_seventy}");
}

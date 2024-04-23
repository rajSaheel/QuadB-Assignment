fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        let median = (arr[mid - 1] + arr[mid]) as f64 / 2.0;
        median
    } else {
        let mid = len / 2;
        let median = arr[mid] as f64;
        median
    }
}

fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in (i + 1)..arr.len() {
            if arr[min] > arr[j] {
                min = j;
            }
        }
        let temp = arr[i];
        arr[i] = arr[min];
        arr[min] = temp;
    }
}
 
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && (arr[j - 1] > arr[j]) {
            let temp = arr[j];
            arr[j] = arr[j - 1];
            arr[j - 1] = temp;
            j -= 1;
        }
    }
}
 
#[cfg(test)]
mod tests {
 
    use super::*;
 
    #[test]
    fn selection_sort_test() {
        let mut arr: [i32; 9] = [77, 89, 74, 68, 70, 49, 5, 62, 51];
        selection_sort(&mut arr);
        assert_eq!(arr, [5, 49, 51, 62, 68, 70, 74, 77, 89]);
    }
 
    #[test]
    fn insertion_sort_test() {
        let mut arr: [i32; 9] = [77, 89, 74, 68, 70, 49, 5, 62, 51];
        insertion_sort(&mut arr);
        assert_eq!(arr, [5, 49, 51, 62, 68, 70, 74, 77, 89]);
    }
}
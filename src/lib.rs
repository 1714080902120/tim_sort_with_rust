fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let n1 = mid - left + 1;
    let n2 = right - mid;
    let mut left_arr = vec![arr[left]; n1];
    let mut right_arr = vec![arr[mid + 1]; n2];

    for i in 0..n1 {
        left_arr[i] = arr[left + i];
    }

    for j in 0..n2 {
        right_arr[j] = arr[mid + 1 + j];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < n1 && j < n2 {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

fn timsort<T: Ord + Copy>(arr: &mut [T], run: usize) {
    let n = arr.len();
    for i in (0..n).step_by(run) {
        let end = std::cmp::min(i + run - 1, n - 1);
        insertion_sort(&mut arr[i..=end]);
    }

    let mut size = run;
    while size < n {
        for left in (0..n).step_by(2 * size) {
            let mid = std::cmp::min(left + size - 1, n - 1);
            let right = std::cmp::min(left + 2 * size - 1, n - 1);
            merge(arr, left, mid, right);
        }
        size *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timsort() {
        let mut arr1 = [5, 2, 7, 1, 9, 4, 8, 3, 6];
        let mut arr2 = [3, 2, 1];
        let mut arr3 = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut arr4 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut arr5 = [1, 3, 2, 5, 4, 7, 6, 9, 8, 10];
        
        let arr1_len = arr1.len();
        let arr2_len = arr2.len();
        let arr3_len = arr3.len();
        let arr4_len = arr4.len();
        let arr5_len = arr5.len();

        timsort(&mut arr1, arr1_len);
        timsort(&mut arr2, arr2_len);
        timsort(&mut arr3, arr3_len);
        timsort(&mut arr4, arr4_len);
        timsort(&mut arr5, arr5_len);
    
        assert_eq!(arr1, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(arr2, [1, 2, 3]);
        assert_eq!(arr3, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(arr4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(arr5, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    
}

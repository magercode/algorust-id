pub fn bubble_sort(mut data: Vec<i32>) -> Vec<i32> {
    let n = data.len();
    if n < 2 {
        return data;
    }

    for i in 0..n {
        let mut swapped = false;
        for j in 0..(n - i - 1) {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    data
}

pub fn merge_sort(data: &[i32]) -> Vec<i32> {
    if data.len() <= 1 {
        return data.to_vec();
    }

    let mid = data.len() / 2;
    let left = merge_sort(&data[..mid]);
    let right = merge_sort(&data[mid..]);
    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out.push(left[i]);
            i += 1;
        } else {
            out.push(right[j]);
            j += 1;
        }
    }

    out.extend_from_slice(&left[i..]);
    out.extend_from_slice(&right[j..]);
    out
}

pub fn quick_sort(data: &mut [i32]) {
    if data.len() <= 1 {
        return;
    }
    let pivot = partition(data);
    let (left, right) = data.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(data: &mut [i32]) -> usize {
    let len = data.len();
    let pivot_idx = len - 1;
    let pivot = data[pivot_idx];
    let mut i = 0;
    for j in 0..pivot_idx {
        if data[j] <= pivot {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, pivot_idx);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting_works() {
        let input = vec![9, 3, 7, 1, 5];
        assert_eq!(bubble_sort(input.clone()), vec![1, 3, 5, 7, 9]);
        assert_eq!(merge_sort(&input), vec![1, 3, 5, 7, 9]);

        let mut quick = input;
        quick_sort(&mut quick);
        assert_eq!(quick, vec![1, 3, 5, 7, 9]);
    }
}

fn merge_sort(data: &[i32]) -> Vec<i32> {
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
    let (mut i, mut j) = (0usize, 0usize);
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

fn main() {
    let input = vec![8, 3, 1, 9, 4];
    println!("{:?}", merge_sort(&input));
}

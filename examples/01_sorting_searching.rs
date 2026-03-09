use algorust::algorithms::searching::{binary_search, linear_search};
use algorust::algorithms::sorting::{bubble_sort, merge_sort, quick_sort};

fn main() {
    let data = vec![42, 7, 13, 9, 100, 3];
    println!("Data awal: {data:?}");

    let bubble = bubble_sort(data.clone());
    let merge = merge_sort(&data);
    let mut quick = data.clone();
    quick_sort(&mut quick);

    println!("Bubble sort: {bubble:?}");
    println!("Merge sort : {merge:?}");
    println!("Quick sort : {quick:?}");

    let target = 13;
    println!("Linear search {target}: {:?}", linear_search(&data, target));
    println!(
        "Binary search {target}: {:?}",
        binary_search(&quick, target)
    );
}

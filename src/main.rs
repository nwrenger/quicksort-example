fn main() {
    let mut array: Vec<i32> = vec![
        -100, 200, 1, 6, 7, 5, 23, 9, 1000000000, 3232, 323, 4, 3, 45, 5, 456, -34, 43,
    ];
    quicksort(&mut array);
    println!("{:?}", array);
}

fn quicksort(a: &mut [i32]) {
    let pivot = if let Some (&pivot) =  a.last() {
        pivot
    } else {
        return;
    };
    let mut index = 0;
    for current in 0..a.len() - 1 {
        if a[current] < pivot {
            a.swap(index, current);
            index += 1;
        }
    }
    let last = a.len() - 1;
    a.swap(index, last);
    let (lower, upper) = a.split_at_mut(index);
    quicksort(lower);
    quicksort(&mut upper[1..]); // ignoring pivot element
}

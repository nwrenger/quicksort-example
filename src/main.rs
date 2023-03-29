fn main() {
    let array: Vec<i32> = vec![
        -100, 200, 1, 6, 7, 5, 23, 9, 1000000000, 3232, 323, 4, 3, 45, 5, 456, -34, 43,
    ];
    println!("{:?}", quicksort(array));
}

fn quicksort(a: Vec<i32>) -> Vec<i32> {
    let mut sorted = false;
    for pair in a.windows(2) {
        if pair[0] > pair[1] {
            sorted = true;
            break;
        } else {
            sorted = false;
        }
    }
    if sorted {
        let pivot = if let Some(&pivot) = a.last() {
            pivot
        } else {
            return a;
        };
        let mut smaller = vec![];
        let mut bigger = vec![];
        for &x in &a[0..a.len() - 1] {
            if x < pivot {
                smaller.push(x)
            } else {
                bigger.push(x)
            }
        }
        let mut b: Vec<i32> = vec![];
        b.append(&mut quicksort(smaller));
        b.push(pivot);
        b.append(&mut quicksort(bigger));
        b
    } else {
        return a;
    }
}

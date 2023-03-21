fn main() {
    let array: Vec<i32> = vec![
        -100, 200, 1, 6, 7, 5, 23, 9, 1000000000, 3232, 323, 4, 3, 45, 5, 456, -34, 43,
    ];
    printlist(quicksort(array))
}

fn quicksort(a: Vec<i32>) -> Vec<i32> {
    let pivot = if a.last().is_some() {
        a[0]
    } else {
        return a;
    };
    let mut smaller = vec![];
    let mut nor = vec![];
    let mut bigger = vec![];
    for x in a {
        if x < pivot {
            smaller.push(x)
        }
        if x > pivot {
            bigger.push(x)
        }
        if x == pivot {
            nor.push(x)
        }
    }
    let mut b: Vec<i32> = vec![];
    b.append(&mut quicksort(smaller));
    b.append(&mut nor);
    b.append(&mut quicksort(bigger));
    b
}

fn printlist(list: Vec<i32>) {
    for x in list {
        print!("{x} ")
    }
    println!()
}

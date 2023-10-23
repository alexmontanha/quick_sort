
fn main() {
    let mut arr = [5, 4, 3, 2, 1];
    quick_sort(&mut arr);
    println!("{:?}", arr);
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..len]);
}

fn partition<T: Ord>(p0: &mut [T]) -> usize {
    let len: usize = p0.len();
    let pivot = len / 2;
    p0.swap(pivot, len - 1);
    let mut store = 0;
    for i in 0..len - 1 {
        if p0[i] <= p0[len - 1] {
            p0.swap(i, store);
            store += 1;
        }
    }
    p0.swap(store, len - 1);
    store
}
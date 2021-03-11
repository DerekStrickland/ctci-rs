
pub fn sort(items: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot = partition(items, left, right);
    sort(items, left, pivot);
    sort(items, pivot + 1, right);
}

fn partition(items: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = items[(left + right)/2];
    let mut lo = left;
    let mut hi = right;

    loop {
        while items[lo] < pivot {
            lo+=1;
        }
        while items[hi] > pivot {
            hi-=1;
        }
        if lo >= hi {
            return hi;
        }
        items.swap(lo, hi);
    }
}
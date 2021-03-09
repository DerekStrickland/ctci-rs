
pub fn sort(items: Vec<i32>, lo: usize, hi: usize) -> Vec<i32> {
    if lo < hi {
        let (part, items) = partition(items, lo, hi);
        let items = sort(items, lo, part);
        let items = sort(items, part + 1, hi);

        items
    } else {
        items
    }
}

fn partition(mut items: Vec<i32>, lo: usize, hi: usize) -> (usize, Vec<i32>) {
    let pivot = items[((hi + lo) / 2) as usize];
    let mut i = lo - 1;
    let j = hi + 1;

    loop {
        while items[i] < pivot {
            i = i + 1;
        }
        while items[j] > pivot {
            if i >= j {
                return (j, items);
            } else {
                items[i] = items[j];
            }
        }
    }
}


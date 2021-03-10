pub fn sort(items: &mut Vec<i32>, left: usize, right: usize) {
    let index = partition(items, left, right);
    println!("left {}", left);
    println!("right {}", right);
    println!("index {}", index);

    if left < (index - 1) {
        sort(items, left, index - 1);
    }

    if index < right {
        sort(items, index, right);
    }
}

fn partition(items: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let mut left = left;
    let mut right = right;

    let pivot_index = (left + (right - left)) / 2;
    println!("pivot_index {}", pivot_index);
    let pivot = items[pivot_index];
    println!("pivot {}", pivot);
    while left <= right {
        while items[left] < pivot {
            left += 1;
        }

        //println!("left evaluated to {}", items[left]);

        while items[right] > pivot {
            right -= 1;
        }

        println!("right evaluated to {}", items[right]);

        if left <= right {
            println!("before swap {} - {}", items[left], items[right]);
            items.swap(left, right);
            println!("after swap {} - {}", items[left], items[right]);
            left += 1;
            right -= 1;
        }
    }

    println!("returning left {}", left);
    left
}

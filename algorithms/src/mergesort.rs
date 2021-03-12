// Conceptually, a merge sort works as follows:
// Divide the unsorted list into n sublists each containing one element.
// A list of one element is considered sorted.
// Repeatedly merge sublists to produce new sorted sublists until there is
// only one sublist remaining. This will be the sorted list.
pub fn top_down_sort(items: &mut [i32; 100]) {
    let mut buffer: [i32; 100] = [0; 100];
    buffer.copy_from_slice(&items[..]);
    top_down_split_merge(&mut buffer, 0, 99, items);
}

fn top_down_split_merge(buffer:&mut [i32;100], start: usize, end:usize, items:&mut [i32;100]) {
    if (end - start) <= 1 {
        return;
    }

    let middle = (start + end)/2;

    top_down_split_merge(items, start, middle, buffer);
    top_down_split_merge(items, middle, end, buffer);

    top_down_merge(buffer, start, middle, end, items);
}

fn top_down_merge(buffer:&mut [i32;100], start:usize, middle:usize, end:usize, items:&mut [i32;100]) {
    println!("start {}", start);
    println!("middle {}", middle);
    println!("end {}", end);

    let mut start_index = start;
    let mut middle_index = middle;
    let ubound = end + 1;

    println!("ubound {}", ubound);

    for index in start..ubound {
        println!("index {}", index);
        if start_index < middle && (middle_index >= ubound || buffer[start_index] <= buffer[middle_index]) {
            println!("start_index {}", start_index);
            println!("middle {}", middle);
            println!("middle_index {}", middle_index);
            println!("ubound {}", ubound);

            items[index] = buffer[start_index];
            start_index +=1;
        } else {
            println!("middle_index {}", middle_index);
            items[index] = buffer[middle_index];
            middle_index +=1;
        }
    }
}


pub fn bottom_up_sort(items: &mut [i32; 100]) {
    let mut buffer: [i32;100] = [0;100];
    buffer.copy_from_slice(&items[..]);



}
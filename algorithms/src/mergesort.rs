use std::cmp;
// Conceptually, a merge sort works as follows:
// Divide the unsorted list into n sublists each containing one element.
// A list of one element is considered sorted.
// Repeatedly merge sublists to produce new sorted sublists until there is
// only one sublist remaining. This will be the sorted list.
pub fn top_down_sort(items: &mut [i32; 100]) {
    let mut buffer: [i32; 100] = [0;100];
    buffer.copy_from_slice(&items[..]);
    top_down_split_merge(&mut buffer, 0, items.len(), items);
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
    let mut start_index = start;
    let mut middle_index = middle;

    for index in start..end {
        if start_index < middle && (middle_index >= end || buffer[start_index] <= buffer[middle_index]) {
            items[index] = buffer[start_index];
            start_index +=1;
        } else {
            items[index] = buffer[middle_index];
            middle_index +=1;
        }
    }
}


pub fn bottom_up_sort(items: &mut [i32; 100]) {
    let mut buffer: [i32;100] = [0;100];
    let mut width = 1;

    while width < items.len() {
        let mut index = 0;

        while index < items.len() {
            bottom_up_merge(items, index, cmp::min(index + width, items.len()), cmp::min(index+2*width, items.len()), &mut buffer);
            index = index + 2 * width;
        }

        items.copy_from_slice(&buffer[..]);
        width = 2 * width;
    }
}

fn bottom_up_merge(items: &mut [i32;100], start: usize, middle: usize, end: usize, buffer: &mut [i32;100]) {
    let mut start_index = start;
    let mut middle_index = middle;
    let mut loop_index = start;

    while loop_index < end {
        if start_index < middle && (middle_index >= end || items[start_index] <= items[middle_index]) {
            buffer[loop_index] = items[start_index];
            start_index+=1;
        } else {
            buffer[loop_index] = items[middle_index];
            middle_index+=1;
        }
        loop_index+=1;
    }
}
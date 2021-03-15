mod quicksort;
mod mergesort;

#[cfg(test)]
mod tests {
    use crate::quicksort;
    use crate::mergesort;
    use rand::Rng;
    use std::convert::TryInto;

    fn cut_deck() -> Vec<i32> {
        let mut index: usize = 0;
        let mut buffer: [i32; 100] = [0; 100];

        while index < 100 {
            buffer[index] = (index + 1) as i32;
            index = index + 1;
        }

        let mut rng = rand::thread_rng();
        let mut fulcrum = rng.gen_range(1..100);

        while fulcrum == 100 {
            fulcrum = rng.gen_range(1..100);
        }

        let mut result: Vec<i32> = Vec::with_capacity(100);

        result.extend_from_slice(&buffer[fulcrum..]);
        result.extend_from_slice(&buffer[..fulcrum]);

        result
    }

    fn cut_deck2() -> [i32; 100]  {
        let mut index: usize = 0;
        let mut buffer: [i32; 100] = [0; 100];

        while index < 100 {
            buffer[index] = (index + 1) as i32;
            index = index + 1;
        }

        let mut rng = rand::thread_rng();
        let mut fulcrum = rng.gen_range(1..100);

        while fulcrum == 100 || fulcrum == 1 {
            fulcrum = rng.gen_range(1..100);
        }

        println!("fulcrum is {}", fulcrum);

        let result: [i32; 100];

        result = [&buffer[fulcrum..], &buffer[..fulcrum]].concat().try_into().unwrap();

        result
    }

    #[test]
    fn test_cut_deck() {
        let cut = cut_deck();

        for v in cut.iter() {
            println!("{}", v);
        }

        assert_eq!(100, cut.len());
    }

    #[test]
    fn test_cut_deck2() {
        let cut = cut_deck2();

        for v in cut.iter() {
            println!("{}", v);
        }

        assert_eq!(100, cut.len());
    }

    #[test]
    fn test_quicksort() {
        let mut items = cut_deck();

        let len = items.len() - 1;
        println!("len {}", len);
        quicksort::sort(&mut items, 0, len);

        let mut index: usize = 0;
        while index < 100 {
            println!("test_quicksort iteration {}, element: {}", index, items[index]);
            assert_eq!(items[index] - 1, (index as i32));
            index += 1;
        }
    }

    #[test]
    fn test_top_down_mergesort() {
        let mut items = cut_deck2();
        let mut index: usize = 0;

        while index < 100 {
            println!("items {}, element value: {}", index, items[index]);
            index+=1;
        }

        index = 0;

        mergesort::top_down_sort(&mut items);

        while index < items.len() {
            println!("test_top_down_mergesort iteration {}, element value: {}", index, items[index]);
            assert_eq!(items[index], (index as i32)+1);
            index += 1;
        }
    }

    // #[test]
    // fn test_bottom_up_mergesort() {
    //     let mut items = cut_deck2();
    //     mergesort::top_down_sort(&mut items);
    //
    //     let mut index: usize = 0;
    //     while index < 100 {
    //         println!("iteration {}, element: {}", index, items[index]);
    //         assert_eq!(items[index] - 1, (index as i32));
    //         index += 1;
    //     }
    // }
}

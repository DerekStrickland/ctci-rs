mod quicksort;

#[cfg(test)]
mod tests {
    use crate::quicksort;
    use rand::Rng;

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

    #[test]
    fn test_cut_deck() {
        let cut = cut_deck();

        for v in cut {
            println!("{}", v);
        }
    }

    #[test]
    fn test_quicksort() {
        let mut items = cut_deck();

        let len = items.len() - 1;
        println!("len {}", len);
        quicksort::sort(&mut items, 0, len);

        let mut index: usize = 0;
        while index < 100 {
            println!("iteration {}, element: {}", index, items[index]);
            assert_eq!(items[index] - 1, (index as i32));
            index += 1;
        }
    }
}

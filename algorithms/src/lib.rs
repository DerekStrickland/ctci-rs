mod quicksort;

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::quicksort;

    fn cut_deck() -> Vec<i32> {
        let ubound: usize = 1000;
        let mut index: usize = 0;
        let mut buffer: [i32; 1000] = [0; 1000];

        while index < ubound {
            buffer[index] = (index + 1) as i32;
            index = index + 1;
        }

        let mut rng = rand::thread_rng();
        let mut fulcrum = rng.gen_range(1..1000);

        while fulcrum == ubound {
            fulcrum = rng.gen_range(1..1000);
        }

        let mut result:Vec<i32> = Vec::new();

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
        let items = cut_deck();

        let len = items.len() - 1;
        let items = quicksort::sort(items, 1, len);

        let mut index: usize = 0;
        while index < 1000 {
            assert_eq!(*items.get(index).unwrap(), (index as i32) + 1);
            index = index + 1;
        }
    }
}

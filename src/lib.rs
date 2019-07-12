#![deny(warnings)]

pub fn quicksort<T: Clone>(array: &[T]) -> Vec<T>
where
    T: PartialOrd<T>,
{
    if array.is_empty() {
        Vec::new()
    } else {
        let v = array[0].clone();
        let mut left = quicksort(
            &array[1..]
                .iter()
                .cloned()
                .filter(|x| x < &v)
                .collect::<Vec<T>>(),
        );
        let right = quicksort(
            &array[1..]
                .iter()
                .cloned()
                .filter(|x| x >= &v)
                .collect::<Vec<T>>(),
        );
        left.push(v);
        left.extend_from_slice(&right);
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{arbitrary::any, collection::vec, proptest};

    fn is_ordered<T: Ord>(array: &[T]) -> bool {
        array.windows(2).all(|w| w[0] <= w[1])
    }

    proptest! {
        #[test]
        fn result_is_ordered(array in vec(any::<u8>(), 0_usize..1024)) {
            assert!(is_ordered(&quicksort(&array)));
        }

        #[test]
        fn result_is_same_size(array in vec(any::<u8>(), 0_usize..1024)) {
            assert_eq!(quicksort(&array).len(), array.len());
        }

        #[test]
        fn result_matches_sort_result(array in vec(any::<u8>(), 0_usize..1024)) {
            let mut clone = array.clone();
            clone.sort();
            assert_eq!(quicksort(&array), clone);
        }
    }
}

pub enum SortingOrder {
    Ascending,
    Descending,
}

pub fn bubble_sort<T: PartialOrd>(v: &mut [T], order: SortingOrder) {
    for i in 0 ..v.len() {
        let mut sorted = true;
        for j in 0..v.len() - i - 1 {
            match order {
                SortingOrder::Ascending => {
                    if v[j] > v[j + 1] {
                        v.swap(j, j + 1);
                        sorted = false;
                    }
                },
                SortingOrder::Descending => {
                    if v[j] < v[j + 1] {
                        v.swap(j, j + 1);
                        sorted = false;
                    }
                }
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut v, SortingOrder::Ascending);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
        bubble_sort(&mut v, SortingOrder::Descending);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }
}
fn insertion_sort_ascending(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let n = list.len();

    for i in 1..n {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }

    list
}

fn insertion_sort_descending(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let n = list.len();

    for i in 1..n {
        let mut j = i;
        while j > 0 && list[j - 1] < list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }

    list
}

fn main() {
}

#[test]
fn test_insertion_sort_ascending() {
    let mut input = vec![(3, 1), (4, 1), (5, 9), (6, 5), (3, 7)];
    let result = insertion_sort_ascending(input.clone());
    input.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    assert_eq!(result, input);
}

#[test]
fn test_insertion_sort_descending() {
    let mut input = vec![(3, 1), (4, 1), (5, 9), (6, 5), (3, 7)];
    let result = insertion_sort_descending(input.clone());
    input.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        } else {
            b.1.cmp(&a.1)
        }
    });
    assert_eq!(result, input);
}

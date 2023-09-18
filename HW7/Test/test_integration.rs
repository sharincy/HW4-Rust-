
#[test]
fn test_sorting_asc() {
    let mut input = vec![5, 2, 9, 1, 5];
    let expected = vec![1, 2, 5, 5, 9];
    input.sort_by(|a, b| a.cmp(b));
    assert_eq!(input, expected);
}

#[test]
fn test_sorting_desc() {
    let mut input = vec![5, 2, 9, 1, 5];
    let expected = vec![9, 5, 5, 2, 1];
    input.sort_by(|a, b| b.cmp(a));
    assert_eq!(input, expected);
}

//----


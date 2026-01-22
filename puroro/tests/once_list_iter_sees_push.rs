use ::once_list2::OnceListWithTailLen as OnceList;
#[test]
fn once_list_iter_sees_push_after_exhausted() {
    let list = OnceList::<i32>::new();
    list.push(1);

    let mut it = list.iter();
    assert_eq!(it.next().copied(), Some(1));
    assert_eq!(it.next().copied(), None);

    // After the iterator reached the end, pushing a new element should make it visible
    // from the same iterator. This property is relied upon by `LazyRepeatedIter`.
    list.push(2);
    assert_eq!(it.next().copied(), Some(2));
    assert_eq!(it.next().copied(), None);
}

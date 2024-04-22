use crate::ALLOCATOR;

#[test_case]
fn memory1() {
    let mut test = alloc::vec![1, 2, 3];
    assert_eq!(test[0], 1);
    assert_eq!(test[1], 2);
    assert_eq!(test[2], 3);
    test.push(3);
    assert_eq!(test[0], 1);
    assert_eq!(test[1], 2);
    assert_eq!(test[2], 3);
    assert_eq!(test[3], 3);
    // check allocator

    let test2 = alloc::vec![-1, -2, -3, -4];
    assert_eq!(test2[0], -1);
    assert_eq!(test2[1], -2);
    assert_eq!(test2[2], -3);
    assert_eq!(test2[3], -4);

    assert_eq!(test[0], 1);
    assert_eq!(test[1], 2);
    assert_eq!(test[2], 3);
    assert_eq!(test[3], 3);
    // check allocator

    drop(test2);
    drop(test);
    // check allocator
}

#[test_case]
fn memory2() {
    let mut test = alloc::vec::Vec::new();
    for i in 0..4 {
        let n = i * 4;
        test.push(alloc::vec![n, n + 1, n + 2, n + 3]);
    }
    for i in 0..4 {
        let n = i * 4;
        assert_eq!(test[i][0], n);
        assert_eq!(test[i][1], n + 1);
        assert_eq!(test[i][2], n + 2);
        assert_eq!(test[i][3], n + 3);
    }
    // check allocator

    drop(test);
    // check allocator
}

#[test_case]
fn memory_reuse() {
    for _ in 0..1000 {
        let _: alloc::vec::Vec<i32> = alloc::vec::Vec::with_capacity(10_000_0000);
    }
    // check allocator
}

pub fn check_alloc_empty() {
    ALLOCATOR.heap();
}

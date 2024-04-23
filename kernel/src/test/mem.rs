use crate::{StartInfo, ALLOCATOR};

#[test_case]
fn init(_: &StartInfo) {
    check_empty();
}

#[test_case]
fn vec(_: &StartInfo) {
    let mut test = alloc::vec![1, 2, 3];
    assert_eq!(test[0], 1);
    assert_eq!(test[1], 2);
    assert_eq!(test[2], 3);
    test.push(3);
    assert_eq!(test[0], 1);
    assert_eq!(test[1], 2);
    assert_eq!(test[2], 3);
    assert_eq!(test[3], 3);
    check_used(1);

    let test2 = alloc::vec![-1, -2, -3, -4];
    assert_eq!(test2[0], -1);
    assert_eq!(test2[1], -2);
    assert_eq!(test2[2], -3);
    assert_eq!(test2[3], -4);

    assert_eq!(test[0], 1);
    assert_eq!(test[1], 2);
    assert_eq!(test[2], 3);
    assert_eq!(test[3], 3);
    check_used(2);

    drop(test2);
    check_used(1);
    drop(test);
    check_empty();
}

#[test_case]
fn vec_vec(_: &StartInfo) {
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
    check_used(5);

    drop(test);
    check_empty();
}

#[test_case]
fn reuse(info: &StartInfo) {
    let size = info.mem_range.end as usize - info.mem_range.start as usize;
    for _ in 0..10 {
        let _: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(size / 2);
    }
    check_empty();
}

pub fn check_empty() {
    let mut free = 0;
    for _ in ALLOCATOR.heap().iter_free() {
        free += 1;
    }
    let mut total = 0;
    for _ in ALLOCATOR.heap().iter_block() {
        total += 1;
    }
    let used = total - free;
    assert_eq!((free, used), (1, 0), "(free, used)");
}

pub fn check_used(num: usize) {
    let mut free = 0;
    for _ in ALLOCATOR.heap().iter_free() {
        free += 1;
    }
    let mut total = 0;
    for _ in ALLOCATOR.heap().iter_block() {
        total += 1;
    }
    let used = total - free;
    assert_eq!(used, num);
}

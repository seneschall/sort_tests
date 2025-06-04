pub fn merge_sort<T>(list: &mut [T])
where
    T: Ord + Clone + Default + std::fmt::Debug,
{
    let length = list.len();
    let mut result_buf = vec![T::default(); length]; // buffer the result will be written to
                                                     // sort(list, 0, length / 2, length);

    sort(list, 0, length, &mut result_buf);

    for i in 0..length {
        list[i] = result_buf[i].clone();
    }
}

fn sort<T>(list: &[T], lo: usize, hi: usize, result_buf: &mut [T])
where
    T: Ord + Clone + Default + std::fmt::Debug,
{
    if hi - lo <= 1 {
        return;
    }

    let mid: usize = (lo + hi) / 2;

    sort(list, lo, mid, result_buf);
    sort(list, mid, hi, result_buf);
    merge(list, lo, mid, hi, result_buf);
    println!("{result_buf:?}");
}

fn merge<T>(list: &[T], lo: usize, mid: usize, hi: usize, result_buf: &mut [T])
where
    T: Ord + Clone + Default + std::fmt::Debug,
{
    let mut buf_idx = lo;

    let mut left = list[lo..=mid].iter();
    let mut right = list[mid + 1..hi].iter();

    let mut l = left.next();
    let mut r = left.next();

    loop {
        match (l, r) {
            (None, None) => break,
            (Some(a), None) => {
                result_buf[buf_idx] = a.clone();
                buf_idx += 1;
                break;
            }
            (None, Some(b)) => {
                result_buf[buf_idx] = b.clone();
                buf_idx += 1;
                break;
            }
            (Some(a), Some(b)) => {
                if a <= b {
                    result_buf[buf_idx] = a.clone();
                    buf_idx += 1;
                    l = left.next();
                } else {
                    result_buf[buf_idx] = b.clone();
                    buf_idx += 1;
                    r = right.next();
                }
            }
        }
    }

    for elem in left {
        result_buf[buf_idx] = elem.clone();
        buf_idx += 1;
    }

    for elem in right {
        result_buf[buf_idx] = elem.clone();
        buf_idx += 1;
    }

    // while lft_idx <= mid && rgt_idx < hi {
    //     if list[lft_idx] <= list[rgt_idx] {
    //         result_buf[buf_idx] = list[lft_idx].clone();
    //         lft_idx += 1;
    //     } else {
    //         result_buf[buf_idx] = list[rgt_idx].clone();
    //         rgt_idx += 1;
    //     }
    // }
}

fn main() {
    let mut lst = vec![5, 4, 3, 2, 1];
    println!("Before: {:?}", lst);
    merge_sort(&mut lst);
    println!("After: {:?}", lst);
}

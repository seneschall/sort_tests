use core::panic;

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn sort_ints(xs: Vec<i32>) -> bool {
        let mut ys = xs.clone();
        let mut xs = xs.clone();
        ys.sort_unstable();
        merge_sort(&mut xs);
        xs == ys
    }
}

pub fn merge_sort<T>(list: &mut [T])
where
    T: Ord + Clone + Default + std::fmt::Debug,
{
    sort(list);
}

fn sort<T>(list: &mut [T])
where
    T: Ord + Clone + Default + std::fmt::Debug,
{
    if list.len() <= 1 {
        return;
    }

    let mid: usize = list.len() / 2;

    sort(&mut list[..mid]);
    sort(&mut list[mid..]);
    merge(&mut list[..]);
}

fn merge<T>(list: &mut [T])
where
    T: Ord + Clone + Default + std::fmt::Debug,
{
    let mid: usize = list.len() / 2;
    let mut left = list[0..mid].to_vec().into_iter();
    let mut right = list[mid..].to_vec().into_iter();

    let mut a = left.next();
    let mut b = right.next();

    for i in 0..list.len() {
        match (&mut a, &mut b) {
            (None, None) => panic!("Shouldn't happen."),
            (Some(l), None) => {
                list[i] = l.clone();
                a = left.next();
            }
            (None, Some(r)) => {
                list[i] = r.clone();
                b = right.next();
            }
            (Some(l), Some(r)) => {
                if l <= r {
                    list[i] = l.clone();
                    a = left.next();
                } else {
                    list[i] = r.clone();
                    b = right.next();
                }
            }
        }
    }
}

fn main() {
    let mut lst = vec![5, 4, 3, 2, 1];
    println!("Before: {:?}", lst);
    merge_sort(&mut lst);
    println!("After: {:?}", lst);
}

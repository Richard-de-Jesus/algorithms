
pub trait Sort: PartialOrd + Copy {}
impl<T: PartialOrd + Copy> Sort for T {}

pub fn selection_sort<T: Sort>(arr: &mut [T]) {
    
    let len = arr.len();
    if len < 2 {
        return;
    }
    

    for i in 0..len {
        let mut min_index = i;

        for j in i + 1..len {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

pub fn bubble_sort<T: Sort>(arr: &mut [T]) { 
    let len = arr.len();

    if len < 2 {
        return;
    }
    for i in 0..len {
        let mut swapped = false;

        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
// the line j -= 1 was underflowing on the 1st iteration.
// that is why the as usize hack.
pub fn insertion_sort<T: Sort>(arr: &mut [T]) {

    if arr.len() < 2 {
        return;
    }

    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i as isize - 1;

        while j >= 0 && key < arr[j as usize] { 
            arr[j as usize + 1] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key; 
    }
}

fn merge<T: Sort>(arr: &mut [T], mid: usize) {

    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    // indexes of each portion.
    let (mut l, mut r) = (0, 0);

    for val in arr {
        let con1 = r == right.len();
        let con2 = l < left.len();

        if con1 || con2 && left[l] < right[r] {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

pub fn merge_sort<T: Sort>(arr: &mut [T]) {

    if arr.len() > 1 {
        let mid = arr.len() / 2;

        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);

        merge(arr, mid);
    }
}

fn partition<T: Sort>(arr: &mut [T], low: isize, high: isize) -> isize {

    let pivot = high as usize;
    let (mut i, mut j) = (low - 1, high);

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot);
    i
}

fn _quick_sort<T: Sort>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let piv = partition(arr, low, high);

        _quick_sort(arr, low, piv - 1);
        _quick_sort(arr, piv + 1, high);
    }
}
// wrapper over _quick_sort, to give 
// nice interface.
pub fn quick_sort<T: Sort>(arr: &mut [T]) {
    
    let len = arr.len() as isize;
    if len > 1 {
        _quick_sort(arr, 0, len - 1);
    }
}

fn heapfiy<T: Sort>(arr: &mut[T], n: usize, i: usize) {
    
    let mut largest = i;
    // index of left and right portions.
    let (l, r) = (2 * i + 1, 2 * i + 2);

    // if left child exists and is greater than root.
    if l < n && arr[largest] < arr[l] {
        largest = l;
    }
    // same for right child.
    if r < n && arr[largest] < arr[r] {
        largest = r;
    }
    // if needed, change root.
    if largest != i {
        arr.swap(i, largest);

        heapfiy(arr, n, largest);
    }
}


pub fn heap_sort<T: Sort>(arr: &mut [T]) {
    let n = arr.len();

    if n < 2 {
        return;
    }

    let mut i = (n / 2 - 1) as isize;
    while i > -1 {
        heapfiy(arr, n, i as usize);
        i -= 1;
    }
    i = (n - 1) as isize;
    while i > 0 {
        arr.swap(i as usize, 0);
        heapfiy(arr, i as usize, 0);
        i -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let unsorted = [11, 7, 3, 5, 2];
        let expected = [2, 3, 5, 7, 11];
        let mut array = unsorted.clone();

        let foo = &mut array as *mut [usize; 5];

        let test = |f: fn(&mut [_])| {
            unsafe { 
                f(&mut *foo);
                let bar = &expected;
                assert_eq!(bar, & *foo);
                *foo = unsorted.clone();
            }
        };

        test(selection_sort);
        test(bubble_sort);

        test(insertion_sort);
        test(merge_sort);

        test(quick_sort);
        test(heap_sort);
    }
}

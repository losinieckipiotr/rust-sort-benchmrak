use crate::sort::Sort;
use crate::utils;

fn partition_2(vec: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
  // Divides array into two partitions
  // Pivot value
  let num = (hi + lo) / 2;
  let pivot = vec[num as usize]; // The value in the middle of the array

  // Left index
  let mut i = lo - 1;

  // Right index
  let mut j = hi + 1;

  loop {
    // Move the left index to the right at least once and while the element at
    // the left index is less than the pivot
    loop {
      i = i + 1;
      if vec[i as usize] >= pivot {
        break;
      }
    }

    // Move the right index to the left at least once and while the element at
    // the right index is greater than the pivot
    loop {
      j = j - 1;
      if vec[j as usize] <= pivot {
        break;
      }
    }

    // If the indices crossed, return
    if i >= j {
      return j;
    }

    // Swap the elements at the left and right indices
    utils::swap(i as usize, j as usize, vec);
  }
}

fn quick_sort_impl_2(vec: &mut Vec<i32>, lo: i32, hi: i32) {
  // Sorts a (portion of an) array, divides it into partitions, then sorts those
  if lo >= 0 && hi >= 0 && lo < hi {
    let p = partition_2(vec, lo, hi);
    quick_sort_impl_2(vec, lo, p); // Note: the pivot is now included
    quick_sort_impl_2(vec, p + 1, hi);
  }
}

fn quick_sort_2(vec: &mut Vec<i32>) {
  quick_sort_impl_2(vec, 0, (vec.len() - 1) as i32);
}

pub struct QuickSort2 {}

impl Sort for QuickSort2 {
  fn name(&self) -> &str {
    "quick2"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    quick_sort_2(vec);
  }
}

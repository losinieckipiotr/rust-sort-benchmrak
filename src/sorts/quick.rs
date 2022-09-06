use crate::sort::Sort;
use crate::utils::swap;

pub fn partition(vec: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
  // Divides array into two partitions
  let pivot = vec[hi as usize]; // Choose the last element as the pivot

  // Temporary pivot index
  let mut i = lo - 1;

  for j in lo..hi {
    // If the current element is less than or equal to the pivot
    if vec[j as usize] <= pivot {
      // Move the temporary pivot index forward
      i = i + 1;
      // Swap the current element with the element at the temporary pivot index
      // vec.swap(i as usize, j as usize);
      swap(i as usize, j as usize, vec);
    }
  }


  // Move the pivot element to the correct pivot position (between the smaller and larger elements)
  i = i + 1;
  // swap A[i] with A[hi]
  // vec.swap(i as usize, hi as usize);
  swap(i as usize, hi as usize, vec);
  i
}

fn quick_sort_impl(vec: &mut Vec<i32>, lo: i32, hi: i32) {
  // Ensure indices are in correct order
  if lo >= hi || lo < 0 {
    return
  }

  // Partition array and get the pivot index
  let p = partition(vec, lo, hi);

  // Sort the two partitions
  quick_sort_impl(vec, lo, p - 1); // Left side of pivot
  quick_sort_impl(vec, p + 1, hi); // Right side of pivot
}

fn quick_sort(vec: &mut Vec<i32>) {
  quick_sort_impl(vec, 0, (vec.len() - 1) as i32);
}

pub struct QuickSort {}

impl Sort for QuickSort {
  fn name(&self) -> &str {
    "quick"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    quick_sort(vec);
  }
}

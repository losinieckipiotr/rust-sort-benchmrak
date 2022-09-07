use crate::sort::Sort;
use crate::utils::swap;

fn partition_3(vec: &mut Vec<i32>, start: i32, end: i32) -> i32 {
  let pivot = vec[start as usize];
  let mut count = 0;

  for i in (start + 1)..=end {
    if vec[i as usize] <= pivot {
      count = count + 1;
    }
  }

  // Giving pivot element its correct position
  let pivot_index = start + count;
  vec.swap(pivot_index as usize, start as usize);

  // Sorting left and right parts of the pivot element
  let mut i = start;
  let mut j = end;

  while i < pivot_index && j > pivot_index {
    while vec[i as usize] <= pivot {
      i = i + 1;
    }
    while vec[j as usize] > pivot {
      j = j - 1;
    }
    if i < pivot_index && j > pivot_index {
      // vec.swap(i as usize, j as usize);
      swap(i as usize, j as usize, vec);
      i = i + 1;
      j = j - 1;
    }
  }

  return pivot_index;
}

fn quick_sort_impl_3(vec: &mut Vec<i32>, start: i32, end: i32) {
  // base case
  if start >= end {
    return
  }

  // partitioning the array
  let p = partition_3(vec, start, end);

  // Sorting the left part
  quick_sort_impl_3(vec, start, p - 1);
  // Sorting the right part
  quick_sort_impl_3(vec, p + 1, end);
}

fn quick_sort_3(vec: &mut Vec<i32>) {
  quick_sort_impl_3(vec, 0, (vec.len() - 1) as i32);
}

pub struct QuickSort3;

impl Sort for QuickSort3 {
  fn name(&self) -> &str {
    "quick3"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    quick_sort_3(vec);
  }
}

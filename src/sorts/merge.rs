pub fn merge_sort(vec: &mut Vec<i32>) {
  top_down_merge_sort(vec);
}

// Array vec1 has the items to sort; array vec2 is a work array.
fn top_down_merge_sort(vec1: &mut Vec<i32>) {
  // CopyArray(A, 0, n, B);           // one time copy of A[] to B[]
  let mut vec2 = vec1.clone();
  // TopDownSplitMerge(B, 0, n, A);   // sort data from B[] into A[]
  top_down_split_merge(&mut vec2, vec1, 0, vec1.len());
}

// // Split A[] into 2 runs, sort both runs into B[], merge both runs from B[] to A[]
// // iBegin is inclusive; iEnd is exclusive (A[iEnd] is not in the set).
fn top_down_split_merge(vec2: &mut Vec<i32>, vec1: &mut Vec<i32>, begin: usize, end: usize) {
  if end - begin <= 1 {
    return; // consider it sorted
  }
  // split the run longer than 1 item into halves
  let middle = (end + begin) / 2;
  // recursively sort both runs from array A[] into B[]
  top_down_split_merge(vec1, vec2, begin, middle); // sort the left  run
  top_down_split_merge(vec1, vec2, middle, end); // sort the right run
  // merge the resulting runs from array B[] into A[]
  top_down_merge(vec2, vec1, begin, middle, end);
}

//  Left source half is A[ iBegin:iMiddle-1].
// Right source half is A[iMiddle:iEnd-1   ].
// Result is            B[ iBegin:iEnd-1   ].
fn top_down_merge(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>, begin: usize, middle: usize, end: usize) {
  let mut i = begin;
  let mut j = middle;
  // While there are elements in the left or right runs...

  for k in begin..end {
    // If left run head exists and is <= existing right run head.
    if i < middle && (j >= end || vec1[i] <= vec1[j]) {
      vec2[k] = vec1[i];
      i = i + 1;
    } else {
      vec2[k] = vec1[j];
      j = j + 1;
    }
  }
}

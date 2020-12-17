use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;
use core::iter::StepBy;
use std::ops::RangeInclusive;
use std::io::{self, Write};
use std::fs::File;

const TIMES: usize = 2_000;         //how many times do we test each size in order to minimize statistical error
const MIN:   usize = 20;            //min array size
const MAX:   usize = 2_000;         //max array size
const STEP:  usize = 20;            //step for incrementing array size

//Just set this to false if any of this is true:
//1) You don't have ghc installed.
//2) The program panics (the ghc doesn't work well. I had to install ghc-static in order to make it work).
//3) You are not in the mood for jokes.
const HASKELL_JOKE_ENABLED: bool = true;

fn main() {
    let mut rng = rand::thread_rng();

    if HASKELL_JOKE_ENABLED {
        prepare_quick_sort_haskell();
    }

    test(&mut rng);

    let result = worker_gen_sizes(&mut rng);

    save_slice(&gen_sizes().collect::<Vec<_>>(), "../r/sizes.txt");

    save_slice(&result.selection_totals, "../r/selection_totals.txt");
    save_slice(&result.insertion_totals, "../r/insertion_totals.txt");
    save_slice(&result.bubble_totals, "../r/bubble_totals.txt");
    save_slice(&result.merge_totals, "../r/merge_totals.txt");
    save_slice(&result.heap_totals, "../r/heap_totals.txt");
    save_slice(&result.quick_totals, "../r/quick_totals.txt");
    save_slice(&result.quick_worst_totals, "../r/quick_worst_totals.txt");
}

fn test(rng: &mut ThreadRng) {
    let mut vec: Vec<i32> = vec![0; 20];
    let dist = Uniform::new_inclusive(-100, 100);
    for i in vec.iter_mut() {
        *i = dist.sample(rng);
    }

    let vec = vec;
    println!("Array:\n{:?}", vec);

    let mut to_be_heapified: Vec<i32> = vec.clone();
    HeapifiedSlice::heapify(&mut to_be_heapified);
    println!("Heapification:\n{:?}", to_be_heapified);
    fn is_heapified<T: Ord>(slice: &[T], top: usize) -> bool {
        let left = top * 2 + 1;
        let right = top * 2 + 2;
        if left < slice.len() && (slice[left] > slice[top] || !is_heapified(slice, left)) {
            return false;
        }
        if right < slice.len() && (slice[right] > slice[top] || !is_heapified(slice, right)) {
            return false;
        }
        true
    }
    assert!(is_heapified(&to_be_heapified, 0), "heapify is incorrect!");

    let mut vec0: Vec<i32> = vec.clone();
    vec0.sort();
    println!("Default sorted:\n{:?}", vec0);

    let mut vec1: Vec<i32> = vec.clone();
    selection_sort(&mut vec1);
    println!("Selection sorted:\n{:?}", vec1);
    assert!(vec0 == vec1, "Selection sort is incorrect!");

    let mut vec2: Vec<i32> = vec.clone();
    insertion_sort(&mut vec2);
    println!("Insertion sorted:\n{:?}", vec2);
    assert!(vec0 == vec2, "Insertion sort is incorrect!");

    let mut vec3: Vec<i32> = vec.clone();
    bubble_sort(&mut vec3);
    println!("Bubble sorted:\n{:?}", vec3);
    assert!(vec0 == vec3, "Bubble sort is incorrect!");

    let mut vec4: Vec<i32> = vec.clone();
    merge_sort(&mut vec4);
    println!("Merge sorted:\n{:?}", vec4);
    assert!(vec0 == vec4, "Merge sort is incorrect!");

    let mut vec5: Vec<i32> = vec.clone();
    heap_sort(&mut vec5);
    println!("Heap sorted:\n{:?}", vec5);
    assert!(vec0 == vec5, "Heap sort is incorrect!");

    if HASKELL_JOKE_ENABLED {
        let mut vec6: Vec<i32> = vec.clone();
        haskell_quick_sort(&mut vec6);
        println!("Haskell quick sorted:\n{:?}", vec6);
        assert!(vec0 == vec6, "Haskell quick sort is incorrect!");
    }

    let mut vec7: Vec<i32> = vec.clone();
    quick_sort(&mut vec7);
    println!("Quick sorted:\n{:?}", vec7);
    assert!(vec0 == vec7, "Quick sort is incorrect!");

    println!("Tests passed! Either all implementations are correct or all are flawed (including the one from stdlib :D).");
}

fn benchmark<T: Ord, F>(slice: &mut [T], f: F) -> u128
where F: Fn(&mut [T])
{
    let start = Instant::now();

    f(slice);

    let end = Instant::now();
    let time_taken = end.duration_since(start).as_nanos();
    time_taken
}

fn gen_sizes() -> StepBy<RangeInclusive<usize>> {
    let min = if MIN != 0 { MIN } else { MIN + STEP };

    (min..=MAX).step_by(STEP)
}

fn worker_gen_sizes(rng: &mut ThreadRng) -> Result {
    worker(gen_sizes(), rng)
}

fn worker(sizes_iter: StepBy<RangeInclusive<usize>>, rng: &mut ThreadRng) -> Result {
    let dist = Uniform::new_inclusive(i32::MIN, i32::MAX);

    let mut selection_totals: Vec<u128> = Vec::new();
    let mut insertion_totals: Vec<u128> = Vec::new();
    let mut bubble_totals: Vec<u128> = Vec::new();
    let mut merge_totals: Vec<u128> = Vec::new();
    let mut heap_totals: Vec<u128> = Vec::new();
    let mut quick_totals: Vec<u128> = Vec::new();
    let mut quick_worst_totals: Vec<u128> = Vec::new();

    for size in sizes_iter {
        let mut selection_total: u128 = 0;
        let mut insertion_total: u128 = 0;
        let mut bubble_total: u128 = 0;
        let mut merge_total: u128 = 0;
        let mut heap_total: u128 = 0;
        let mut quick_total: u128 = 0;
        let mut quick_worst_total: u128 = 0;

        let mut vec: Vec<i32> = vec![0; size];
        let bad_vec: Vec<i32> = (0..size as i32).rev().collect();

        for _ in 0..TIMES {

            for i in vec.iter_mut() {
                *i = dist.sample(rng);
            }

            selection_total += benchmark(&mut vec.clone(), selection_sort);
            insertion_total += benchmark(&mut vec.clone(), insertion_sort);
            bubble_total += benchmark(&mut vec.clone(), bubble_sort);
            merge_total += benchmark(&mut vec.clone(), merge_sort);
            heap_total += benchmark(&mut vec.clone(), heap_sort);
            quick_total += benchmark(&mut vec.clone(), quick_sort);
            quick_worst_total += benchmark(&mut bad_vec.clone(), quick_sort);
        }

        selection_total /= TIMES as u128;
        insertion_total /= TIMES as u128;
        bubble_total /= TIMES as u128;
        merge_total /= TIMES as u128;
        heap_total /= TIMES as u128;
        quick_total /= TIMES as u128;
        quick_worst_total /= TIMES as u128;

        println!("size = {}, selection = {}, insertion = {}, bubble = {}, merge = {}, heap = {}, quick = {}, worst quick = {}", size, selection_total, insertion_total, bubble_total, merge_total, heap_total, quick_total, quick_worst_total);
        
        selection_totals.push(selection_total);
        insertion_totals.push(insertion_total);
        bubble_totals.push(bubble_total);
        merge_totals.push(merge_total);
        heap_totals.push(heap_total);
        quick_totals.push(quick_total);
        quick_worst_totals.push(quick_worst_total);
    }

    Result { selection_totals, insertion_totals, bubble_totals, merge_totals, heap_totals, quick_totals, quick_worst_totals }
}

fn selection_sort<T: Ord>(slice: &mut [T]) {
    for last_idx in (1..=slice.len()).rev() {
        let mut max_idx: usize = 0;

        for i in 1..last_idx {
            if slice[max_idx] < slice[i] {
                max_idx = i;
            }
        }

        slice.swap(max_idx, last_idx - 1);
    }
}

// MR: I have added two versions:
//
// The first version is about three times faster, but requires the Copy trait.
//
// fn insertion_sort<T: Ord + Copy>(slice: &mut [T]) {
//     for i in 1..slice.len() {
//         let ins_elem_val = slice[i];
//         let mut ins_elem_idx = i;

//         while (ins_elem_idx > 0) && (ins_elem_val < slice[ins_elem_idx - 1]) {
//             slice[ins_elem_idx] = slice[ins_elem_idx - 1];
//             ins_elem_idx -= 1;
//         }

//         slice[ins_elem_idx] = ins_elem_val;
//     }
// }
//
// The second version is slower than the above, but is substantially shorter.
// It might be a bit clearer, but that is a matter of personal taste.

fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut ins_elem_idx = i;

        while (ins_elem_idx > 0) && (slice[i] < slice[ins_elem_idx - 1]) {
            ins_elem_idx -= 1;
        }

        slice[ins_elem_idx..=i].rotate_right(1);
    }
}


fn bubble_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let mut swapped: bool = false;
        for j in 0..(slice.len() - i - 1) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn merge_sort<T: Ord + Copy>(slice: &mut [T]) {

    fn merge<T: Ord + Copy>(left: &[T], right: &[T], buff: &mut Vec<T>) {
        let mut l = 0;
        let mut r = 0;
        while l < left.len() && r < right.len() {
            if right[r] < left[l] {
                buff.push(right[r]);
                r += 1;
            } else {
                buff.push(left[l]);
                l += 1;
            }
        }
        buff.extend_from_slice(&left[l..]);
        buff.extend_from_slice(&right[r..]);
    }

    fn sort<T: Ord + Copy>(slice: &mut [T], buff: &mut Vec<T>) {
        if slice.len() < 2 {
            return;
        } else if slice.len() == 2 {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
        } else {
            let len = slice.len();
            let (left, right) = slice.split_at_mut(len / 2);
            sort(left, buff);
            sort(right, buff);
            merge(left, right, buff);
            slice.copy_from_slice(buff);
            buff.clear();
        }
    }

    let mut buff: Vec<T> = Vec::with_capacity(slice.len());
    sort(slice, &mut buff);
}

struct HeapifiedSlice<'a, T: Ord> {

    slice: &'a mut [T]
}

impl <'a, T: 'a + Ord> HeapifiedSlice<'a, T> {

    //k should not be 0
    fn get_parent(k: usize) -> usize {
        (k - 1) / 2
    }

    fn get_left_child(top: usize) -> usize {
        top * 2 + 1
    }

    fn get_right_child(top: usize) -> usize {
        top * 2 + 2
    }

    fn conditional_swap(slice: &mut [T], largest: &mut usize, child: usize) {
        if child < slice.len() {
            if slice[child] > slice[*largest] {
                *largest = child;
            }
        }
    }

    // fn fix_heap_bottom_to_top(slice: &mut [T], mut idx: usize) {
    //     while idx > 0 {
    //         let parent_idx = Self::get_parent(idx);
    //         if slice[parent_idx] < slice[idx] {
    //             slice.swap(parent_idx, idx);
    //             idx = parent_idx;
    //         } else {
    //             return;
    //         }
    //     }
    // }

    fn fix_heap_top_to_bottom(slice: &mut [T], mut top: usize) {
        loop {
            let mut largest = top;

            Self::conditional_swap(slice, &mut largest, Self::get_left_child(top));
            Self::conditional_swap(slice, &mut largest, Self::get_right_child(top));

            if top != largest {
                slice.swap(top, largest);
                top = largest;
            } else {
                return;
            }
        }
    }
    
    pub fn heapify(slice: &'a mut [T]) -> HeapifiedSlice<'a, T> {
        let last = slice.len() - 1;
        if last > 0 {
            let parent_of_last = Self::get_parent(last);
            for idx in (0..=parent_of_last).rev() {
                Self::fix_heap_top_to_bottom(slice, idx);
            }
        }
        HeapifiedSlice { slice }
    }

    pub fn sort(&mut self) {
        let mut len = self.slice.len();
        while len > 1 {
            self.slice.swap(0, len - 1);
            len -= 1;
            Self::fix_heap_top_to_bottom(&mut self.slice[..len], 0);
        }
    }
}

fn heap_sort<T: Ord>(slice: &mut [T]) {
    HeapifiedSlice::heapify(slice).sort();
}

//WARNING: very stupid programming joke ahead.
fn prepare_quick_sort_haskell() {
    //Everyone knows, that the only way to quicksort something in any other language but Haskell,
    //is to execute the quicksort written in Haskell :)
    let source_file_name = "qsort.hs";
    //I am not really good with Haskell, the `main` function can definitely be simplified. But it works!
    let program = "
main :: IO ()
main = do
  input <- getLine
  let splited = words input
  let ints = map (\\v -> read v :: Integer) splited
  let sorted = qsort ints
  let mapped = map (\\v -> show v ++ \" \") sorted
  mapM_ putStr mapped
  putStrLn \"\"
    
qsort :: (Ord a) => [a] -> [a]
qsort [] = []
qsort (x:xs) = (qsort $ filter (<= x) xs) ++ [x] ++ (qsort $ filter (> x) xs)
    ";
    std::fs::write(source_file_name, program).unwrap();
    //Compile the Haskell source code
    let exit_status = std::process::Command::new("ghc").arg(source_file_name).status().unwrap();
    assert!(exit_status.success(), "Failed to compile Haskell code :-(");
}

//WARNING: this is a continuation of the stupid joke, but now the code is getting even more awfully terrible.
//Haskell code works only for integers, but Rust's stdlib doesn't have a "number" trait (unless we use crate `num_traits`),
//so let's use this weird `ToString + std::str::FromStr` and hope that no one will actually call this function with something that is not a number
fn haskell_quick_sort<T: Ord + ToString + std::str::FromStr>(slice: &mut [T])
where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    use std::io::prelude::*;

    //Serializing our slice
    let mut str = String::new();
    slice.iter().for_each(|i| {
        str.push_str(&i.to_string());
        str.push(' ');
    });
    str.push('\n');

    //Spawning a child
    let mut child = std::process::Command::new("./qsort")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    
    //Getting stdin and stdout of the child
    let mut child_stdin = child.stdin.take().unwrap();
    let child_stdout = child.stdout.take().unwrap();

    //Writing serialized slice to the child's stdin
    child_stdin.write_all(str.as_bytes()).unwrap();

    //Waiting for child to get the job done
    let exit_status = child.wait().unwrap();
    assert!(exit_status.success(), "Failed to execute Haskell's quicksort :-(");

    //Reading the child's output and parsing it back
    let mut reader = io::BufReader::new(child_stdout);
    let mut output = String::new();
    reader.read_line(&mut output).unwrap();
    let trimmed = output.trim();
    trimmed.split(" ").zip(0..slice.len()).for_each(|(s, i)| slice[i] = s.parse::<T>().unwrap());
    //P.S.
    //Currently the perfomance is terrible (who could have thought, right?),
    //I see two ways of improving that:
    //(1) Rewrite the Haskell version to repeteadly read input and output the sorted result for each line,
    //then spawn a child only once (perhaps in the `prepare_quick_sort_haskell` function) and save is somewhere
    //(this will require either using `unsafe` for a global variable or rewriting this function's signature, 
    //which will make it incompatible with `benchmark` function).
    //After that we will have no 'spawn' in this chain: spawn-serialize-write-read-deserialize
    //(2) We can generate C bindings (FFI) from Haskell, and then just call them in Rust.
    //Essentially we will have chain like Rust->C->Haskell, but I think that it might be more performant than the previous option.
    //
    //But this would be taking the joke too far, so I didn't do it :)
}

//Aaand finally that's my actual implementation of quicksort.
fn quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let pivot_idx = 0;
    let mut l = 1;
    let mut r = slice.len() - 1;
    while l < r {
        while l <= r && slice[l] <= slice[pivot_idx] {
            l += 1;
        }
        while r >= l && slice[r] >= slice[pivot_idx] {
            r -= 1;
        }
        if l < r {
            slice.swap(l, r);
        }
    }
    if slice[r] < slice[pivot_idx] {
        slice.swap(pivot_idx, r);
    }
    quick_sort(&mut slice[..r]);
    quick_sort(&mut slice[(r+1)..]);
}

struct Result {

    selection_totals: Vec<u128>,
    insertion_totals: Vec<u128>,
    bubble_totals: Vec<u128>,
    merge_totals: Vec<u128>,
    heap_totals: Vec<u128>,
    quick_totals: Vec<u128>,
    quick_worst_totals: Vec<u128>
}

//Save slice to a file and print them to stdout
fn save_slice<T: ToString>(slice: &[T], file_name: &str) {
    let mut string = slice.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    string.push('\n');
    io::stdout().write_all(string.as_bytes()).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(string.as_bytes()).unwrap();
}

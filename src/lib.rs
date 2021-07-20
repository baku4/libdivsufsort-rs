//! # libdivsufsort-rs
//!
//! `libdivsufsort-rs` is rust wrapper of [`libdivsufsort`](https://github.com/y-256/libdivsufsort)  
//! Including **all APIs** of both **32- and 64-bit** version.
//! - More details are in included in original `C` codes of [`libdivsufsort`](https://github.com/y-256/libdivsufsort)  
//! - I referred to [`pzip-bwt`](https://crates.io/crates/pzip-bwt) crate, which is more simpler version for wrapping around bwt function of `libdivsufsort`

mod divsufsort;
mod divsufsort64;

use std::ffi::CStr;

/// "Constructs the suffix array of a given string."  
/// Input: Vector of bytes  
/// Output: Suffix array
pub fn divsufsort(input_string: &Vec<u8>) -> Option<Vec<i32>> {
    let string_length = input_string.len();
    let mut sa: Vec<i32> = vec![0; string_length];
    let err = unsafe {
        divsufsort::divsufsort(
            input_string.as_ptr(),
            sa.as_mut_ptr(),
            string_length as i32,
        )
    };
    if err == 0 {
        Some(sa)
    } else {
        None
    }
}
/// 64-bit version of [divsufsort]
pub fn divsufsort64(input_string: &Vec<u8>) -> Option<Vec<i64>> {
    let string_length = input_string.len();
    let mut sa: Vec<i64> = vec![0; string_length];
    let err = unsafe {
        divsufsort64::divsufsort64(
            input_string.as_ptr(),
            sa.as_mut_ptr(),
            string_length as i64,
        )
    };
    if err == 0 {
        Some(sa)
    } else {
        None
    }
}

/// "Constructs the burrows-wheeler transformed string of a given string."  
/// Input: Vector of bytes  
/// A input vector is transformed to burrows wheeler transformed string  
/// Output: Primary index(usually $ sign) of burrows wheeler transformed string
pub fn divbwt(input_string: &mut Vec<u8>) -> Option<i32> {
    let string_length = input_string.len();
    let mut temp_array: Vec<i32> = vec![0; string_length];
    let primary_index =  unsafe {
        divsufsort::divbwt(
            input_string.as_ptr(),
            input_string.as_mut_ptr(),
            temp_array.as_mut_ptr(),
            string_length as i32,
        )
    };
    if primary_index >= 0 {
        Some(primary_index)
    } else {
        None
    }
}
/// 64-bit version of `divbwt`
pub fn divbwt64(input_string: &mut Vec<u8>) -> Option<i64> {
    let string_length = input_string.len();
    let mut temp_array: Vec<i64> = vec![0; string_length];
    let primary_index = unsafe {
        divsufsort64::divbwt64(
            input_string.as_ptr(),
            input_string.as_mut_ptr(),
            temp_array.as_mut_ptr(),
            string_length as i64,
        )
    };
    if primary_index >= 0 {
        Some(primary_index)
    } else {
        None
    }
}

/// "Returns the version of the divsufsort library."
pub fn divsufsort_version() -> String {
    unsafe {
        let ptr = divsufsort::divsufsort_version();
        let cstr = CStr::from_ptr(ptr).to_str();
        cstr.unwrap().to_string()
    }
}
/// 64-bit version of [divsufsort_version]
pub fn divsufsort64_version() -> String {
    unsafe {
        let ptr = divsufsort64::divsufsort64_version();
        let cstr = CStr::from_ptr(ptr).to_str();
        cstr.unwrap().to_string()
    }
}

/// "Constructs the burrows-wheeler transformed string of a given string and suffix array."  
/// Input: Vector of bytes & its suffix array  
/// A input vector is transformed to burrows wheeler transformed string  
/// Output: Primary index(usually $ sign) of burrows wheeler transformed string
pub fn bw_transform(input_string: &mut Vec<u8>, suffix_array: &mut Vec<i32>) -> Option<i32> {
    let string_length = input_string.len();
    let mut primary_index: i32 = 0;
    let err = unsafe {
        divsufsort::bw_transform(
            input_string.as_ptr(),
            input_string.as_mut_ptr(),
            suffix_array.as_mut_ptr(),
            string_length as i32,
            &mut primary_index,
        )
    };
    if err == 0 {
        Some(primary_index)
    } else {
        None
    }
}
/// 64-bit version of [bw_transform]
pub fn bw_transform64(input_string: &mut Vec<u8>, suffix_array: &mut Vec<i64>) -> Option<i64> {
    let string_length = input_string.len();
    let mut primary_index: i64 = 0;
    let err = unsafe {
        divsufsort64::bw_transform64(
            input_string.as_ptr(),
            input_string.as_mut_ptr(),
            suffix_array.as_mut_ptr(),
            string_length as i64,
            &mut primary_index,
        )
    };
    if err == 0 {
        Some(primary_index)
    } else {
        None
    }
}

/// "Inverse BW-transforms a given BWTed string."  
/// Input: Vector of bytes(burrows wheeler transformed) & its primary index  
/// A input vector is transformed to original string
/// Output: If no error occured, get Some value of `unit`. Otherwise, None.
pub fn inverse_bw_transform(input_string: &mut Vec<u8>, primary_index: i32) -> Option<()> {
    let string_length = input_string.len();
    let mut temp_array: Vec<i32> = vec![0; string_length];
    let err = unsafe {
        divsufsort::inverse_bw_transform(
            input_string.as_ptr(),
            input_string.as_mut_ptr(),
            temp_array.as_mut_ptr(),
            string_length as i32,
            primary_index,
        )
    };
    if err == 0 {
        Some(())
    } else {
        None
    }
}
/// 64-bit version of [inverse_bw_transform]
pub fn inverse_bw_transform64(input_string: &mut Vec<u8>, primary_index: i64) -> Option<()> {
    let string_length = input_string.len();
    let mut temp_array: Vec<i64> = vec![0; string_length];
    let err = unsafe {
        divsufsort64::inverse_bw_transform64(
            input_string.as_ptr(),
            input_string.as_mut_ptr(),
            temp_array.as_mut_ptr(),
            string_length as i64,
            primary_index,
        )
    };
    if err == 0 {
        Some(())
    } else {
        None
    }
}

/// "Checks the correctness of a given suffix array."
/// Input: Vector of bytes & its suffix array & verbose option
/// If verbose is true, additional information is printed to stdout.
/// Output: If no error occured, get Some value of `unit`. Otherwise, None.
pub fn sufcheck(input_string: &Vec<u8>, suffix_array: &Vec<i32>, verbose: bool) -> Option<()> {
    let err = unsafe {
        divsufsort::sufcheck(
            input_string.as_ptr(),
            suffix_array.as_ptr(),
            input_string.len() as i32,
            if verbose {1} else {0},
        )
    };
    if err == 0 {
        Some(())
    } else {
        None
    }
}
/// 64-bit version of [sufcheck]
pub fn sufcheck64(input_string: &Vec<u8>, suffix_array: &Vec<i64>, verbose: bool) -> Option<()> {
    let err = unsafe {
        divsufsort64::sufcheck64(
            input_string.as_ptr(),
            suffix_array.as_ptr(),
            input_string.len() as i64,
            if verbose {1} else {0},
        )
    };
    if err == 0 {
        Some(())
    } else {
        None
    }
}

/// "Search for the pattern P in the string T."
/// Input: Vector of bytes & its suffix array and vector of pattern string.
/// Output: tuple of index of suffix array for matched pattern and pattern count
/// Even with multiple counts, only one index is output.
pub fn sa_search(input_string: &Vec<u8>, pattern: &Vec<u8>, suffix_array: &Vec<i32>) -> Option<(i32, i32)> {
    let string_length = input_string.len() as i32;
    let mut idx: i32 = 0;
    let count = unsafe {
        divsufsort::sa_search(
            input_string.as_ptr(),
            string_length,
            pattern.as_ptr(),
            pattern.len() as i32,
            suffix_array.as_ptr(),
            string_length,
            &mut idx
        )
    };
    if count != -1 {
        Some((idx, count))
    } else {
        None
    }
}
/// 64-bit version of [sa_search]
pub fn sa_search64(input_string: &Vec<u8>, pattern: &Vec<u8>, suffix_array: &Vec<i64>) -> Option<(i64, i64)> {
    let string_length = input_string.len() as i64;
    let mut idx: i64 = 0;
    let count = unsafe {
        divsufsort64::sa_search64(
            input_string.as_ptr(),
            string_length,
            pattern.as_ptr(),
            pattern.len() as i64,
            suffix_array.as_ptr(),
            string_length,
            &mut idx
        )
    };
    if count != -1 {
        Some((idx, count))
    } else {
        None
    }
}

/// "Search for the character c in the string T."
/// Input: Vector of bytes & its suffix array and `i32` encoded character.  
/// example of `i32` encoding:
/// ```rust
/// let character: i32 = "T".as_bytes()[0] as i32;
/// ```
/// Output: tuple of index of suffix array for matched pattern and pattern count  
/// Even with multiple counts, only one index is output.  
pub fn sa_simplesearch(input_string: &Vec<u8>, suffix_array: &Vec<i32>, character: i32) -> Option<(i32, i32)> {
    let string_length = input_string.len() as i32;
    let mut idx: i32 = 0;
    let count = unsafe {
        divsufsort::sa_simplesearch(
            input_string.as_ptr(),
            string_length,
            suffix_array.as_ptr(),
            string_length,
            character,
            &mut idx,
        )
    };
    if count != -1 {
        Some((idx, count))
    } else {
        None
    }
}
/// 64-bit version of [sa_simplesearch]
pub fn sa_simplesearch64(input_string: &Vec<u8>, suffix_array: &Vec<i64>, character: i32) -> Option<(i64, i64)> {
    let string_length = input_string.len() as i64;
    let mut idx: i64 = 0;
    let count = unsafe {
        divsufsort64::sa_simplesearch64(
            input_string.as_ptr(),
            string_length,
            suffix_array.as_ptr(),
            string_length,
            character,
            &mut idx,
        )
    };
    if count != -1 {
        Some((idx, count))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use std::thread::sleep;

    fn vec_i32_to_i64(i32_vec: &Vec<i32>) -> Vec<i64> {
        i32_vec.iter().map(|&e| e as i64).collect::<Vec<i64>>()
    }

    #[test]
    fn test_divsufsort() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let suffix_array = divsufsort(&input_string).unwrap();
        let suffix_array_64 = divsufsort64(&input_string).unwrap();
        let ans_64: Vec<i64> = vec![102, 103, 54, 174, 36, 104, 24, 55, 175, 184, 31, 133, 37, 6, 122, 44, 202, 105, 169, 139, 25, 56, 8, 188, 176, 150, 185, 108, 32, 148, 124, 61, 70, 85, 112, 46, 19, 191, 144, 134, 126, 63, 118, 38, 152, 160, 204, 183, 132, 43, 138, 7, 187, 107, 147, 123, 69, 111, 45, 159, 203, 42, 106, 68, 158, 157, 156, 72, 50, 170, 140, 87, 26, 78, 2, 57, 114, 93, 9, 189, 181, 48, 73, 51, 171, 21, 141, 88, 27, 79, 3, 166, 58, 16, 177, 97, 0, 91, 115, 75, 94, 193, 10, 199, 101, 53, 173, 35, 23, 84, 190, 143, 117, 151, 182, 137, 186, 146, 49, 77, 180, 165, 96, 90, 74, 100, 52, 172, 22, 83, 142, 136, 89, 28, 80, 128, 29, 4, 167, 59, 17, 109, 66, 197, 33, 178, 98, 81, 195, 129, 12, 30, 5, 121, 201, 168, 149, 60, 18, 125, 62, 131, 110, 41, 67, 155, 71, 86, 1, 113, 92, 47, 20, 15, 192, 198, 34, 116, 145, 76, 179, 164, 95, 99, 82, 135, 127, 65, 196, 194, 11, 120, 200, 130, 40, 154, 14, 163, 64, 119, 39, 153, 13, 162, 161];

        assert_eq!(vec_i32_to_i64(&suffix_array), ans_64);
        assert_eq!(suffix_array_64, ans_64);
    }

    #[test]
    fn test_divbwt() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let ans_64: (i64, Vec<u8>) = (97, vec![67, 71, 65, 71, 71, 71, 65, 71, 65, 65, 67, 84, 67, 65, 84, 84, 67, 84, 65, 84, 67, 65, 65, 67, 67, 65, 84, 65, 67, 65, 67, 67, 84, 67, 71, 67, 67, 84, 71, 71, 65, 84, 84, 71, 65, 71, 67, 67, 71, 84, 67, 71, 65, 71, 67, 71, 65, 67, 84, 65, 67, 65, 84, 65, 84, 67, 67, 84, 84, 71, 65, 65, 84, 65, 71, 84, 65, 84, 84, 65, 65, 71, 84, 67, 67, 67, 84, 67, 67, 67, 67, 67, 71, 67, 84, 65, 71, 71, 67, 71, 67, 84, 67, 84, 71, 71, 71, 84, 71, 71, 67, 71, 84, 65, 67, 71, 65, 84, 67, 84, 84, 84, 84, 71, 67, 84, 67, 67, 67, 84, 67, 84, 67, 67, 67, 84, 71, 67, 67, 67, 67, 65, 84, 84, 65, 67, 67, 71, 84, 71, 84, 71, 71, 84, 84, 71, 65, 71, 71, 65, 65, 84, 71, 84, 71, 84, 65, 65, 67, 65, 
            67, 65, 65, 84, 65, 71, 71, 67, 65, 67, 71, 84, 67, 71, 71, 65, 65, 84, 71, 67, 67, 84, 67, 71, 84, 84, 84, 84, 65, 65, 65, 65, 71, 84, 65]);

        // 32
        let mut cloned_input = input_string.clone();
        let pidx = divbwt(&mut cloned_input).unwrap();
        assert_eq!((pidx as i64, cloned_input), ans_64);
        
        // 64
        let mut cloned_input = input_string.clone();
        let pidx_64 = divbwt64(&mut cloned_input).unwrap();
        assert_eq!((pidx_64, cloned_input), ans_64);
    }
    
    #[allow(dead_code)]
    fn print_divsufsort_version() {
        // 2.0.1-14-g5f60d6f
        println!("{}", divsufsort_version());
        println!("{}", divsufsort64_version());
    }

    #[test]
    fn test_bw_transform() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();

        // 32
        let bwt_res = {
            let mut cloned_input = input_string.clone();
            let mut sa = divsufsort(&cloned_input).unwrap();
            let pidx = bw_transform(&mut cloned_input, &mut sa).unwrap();
            (cloned_input, pidx)
        };
        let ans = {
            let mut cloned_input = input_string.clone();
            let pidx = divbwt(&mut cloned_input).unwrap();
            (cloned_input, pidx)
        };
        assert_eq!(bwt_res, ans);

        // 64
        let bwt_res = {
            let mut cloned_input = input_string.clone();
            let mut sa = divsufsort64(&cloned_input).unwrap();
            let pidx = bw_transform64(&mut cloned_input, &mut sa).unwrap();
            (cloned_input, pidx)
        };
        let ans = {
            let mut cloned_input = input_string.clone();
            let pidx = divbwt64(&mut cloned_input).unwrap();
            (cloned_input, pidx)
        };
        assert_eq!(bwt_res, ans);
    }

    #[test]
    fn test_inverse_bw_transform() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();

        // 32
        let original_string = {
            let mut bwt = input_string.clone();
            let pidx = divbwt(&mut bwt).unwrap();
            let _ = inverse_bw_transform(&mut bwt, pidx);
            bwt
        };
        assert_eq!(original_string, input_string);

        // 64
        let original_string = {
            let mut bwt = input_string.clone();
            let pidx = divbwt64(&mut bwt).unwrap();
            let _ = inverse_bw_transform64(&mut bwt, pidx);
            bwt
        };
        assert_eq!(original_string, input_string);
    }

    #[test]
    fn test_sufcheck() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();

        // 32
        let err = {
            let suffix_array = divsufsort(&input_string).unwrap();
            sufcheck(&input_string, &suffix_array, false)
        };
        assert_eq!(err, Some(()));

        // 64
        let err = {
            let suffix_array = divsufsort64(&input_string).unwrap();
            sufcheck64(&input_string, &suffix_array, false)
        };
        assert_eq!(err, Some(()));
    }

    #[test]
    fn test_sa_search() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let pattern = "TACACCTGTTTCG".as_bytes().to_vec();

        // 32
        let (position, count) = {
            let suffix_array = divsufsort(&input_string).unwrap();
            let (idx, count) = sa_search(&input_string, &pattern, &suffix_array).unwrap();
            (suffix_array[idx as usize], count)
        };
        assert_eq!((position, count), (5, 1));
        
        // 64
        let (position, count) = {
            let suffix_array = divsufsort64(&input_string).unwrap();
            let (idx, count) = sa_search64(&input_string, &pattern, &suffix_array).unwrap();
            (suffix_array[idx as usize], count)
        };
        assert_eq!((position, count), (5, 1));
    }

    #[test]
    fn test_sa_simplesearch() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let character: i32 = "T".as_bytes()[0] as i32;

        // 32
        let count = {
            let suffix_array = divsufsort(&input_string).unwrap();
            let (_, count) = sa_simplesearch(&input_string, &suffix_array, character).unwrap();
            count
        };
        assert_eq!(count, 54);

        // 64
        let count = {
            let suffix_array = divsufsort64(&input_string).unwrap();
            let (_, count) = sa_simplesearch64(&input_string, &suffix_array, character).unwrap();
            count
        };
        assert_eq!(count, 54);
    }

    #[test]
    fn per_check() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();

        // S1: using SA to bwt
        let (bwt, pidx, sa) = {
            println!("(1) using SA to bwt");
            let mut bwt = input_string.clone();
            let start = Instant::now();
            let sa = divsufsort(&bwt).unwrap();
            println!("sa: {}", start.elapsed().as_nanos());
            let start = Instant::now();
            let mut sa_cloned = sa.clone();
            let pidx = bw_transform(&mut bwt, &mut sa_cloned).unwrap();
            println!("clone & bwt: {}", start.elapsed().as_nanos());
            (bwt, pidx, sa)
        };
        // S2: Sep
        let (bwt, pidx, sa) = {
            println!("(2) Sep");
            let mut bwt = input_string.clone();
            let start = Instant::now();
            let pidx = divbwt(&mut bwt).unwrap();
            println!("bwt: {}", start.elapsed().as_nanos());
            let start = Instant::now();
            let sa = divsufsort(&bwt).unwrap();
            println!("sa: {}", start.elapsed().as_nanos());
            (bwt, pidx, sa)
        };
    }
}
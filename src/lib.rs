mod divsufsort;
mod divsufsort64;

fn divsufsort(input_string: &Vec<u8>) -> Option<Vec<i32>> {
    let string_length = input_string.len();
    let mut sa: Vec<i32> = vec![0; string_length];
    let err: i32;
    unsafe {
        err = divsufsort::divsufsort(
            input_string.as_ptr(),
            sa.as_mut_ptr(),
            string_length as i32,
        );
    }
    if err == 0 {
        Some(sa)
    } else {
        None
    }
}
fn divsufsort64(input_string: &Vec<u8>) -> Option<Vec<i64>> {
    let string_length = input_string.len();
    let mut sa: Vec<i64> = vec![0; string_length];
    let err: i32;
    unsafe {
        err = divsufsort64::divsufsort64(
            input_string.as_ptr(),
            sa.as_mut_ptr(),
            string_length as i64,
        );
    }
    if err == 0 {
        Some(sa)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divsufsort() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let suffix_array = divsufsort(&input_string).unwrap();
        let suffix_array_64 = divsufsort64(&input_string).unwrap();
        let ans: Vec<i64> = vec![102, 103, 54, 174, 36, 104, 24, 55, 175, 184, 31, 133, 37, 6, 122, 44, 202, 105, 169, 139, 25, 56, 8, 188, 176, 150, 185, 108, 32, 148, 124, 61, 70, 85, 112, 46, 19, 191, 144, 134, 126, 63, 118, 38, 152, 160, 204, 183, 132, 43, 138, 7, 187, 107, 147, 123, 69, 111, 45, 159, 203, 42, 106, 68, 158, 157, 156, 72, 50, 170, 140, 87, 26, 78, 2, 57, 114, 93, 9, 189, 181, 48, 73, 51, 171, 21, 141, 88, 27, 79, 3, 166, 58, 16, 177, 97, 0, 91, 115, 75, 94, 193, 10, 199, 101, 53, 173, 35, 23, 84, 190, 143, 117, 151, 182, 137, 186, 146, 49, 77, 180, 165, 96, 90, 74, 100, 52, 172, 22, 83, 142, 136, 89, 28, 80, 128, 29, 4, 167, 59, 17, 109, 66, 197, 33, 178, 98, 81, 195, 129, 12, 30, 5, 121, 201, 168, 149, 60, 18, 125, 62, 131, 110, 41, 67, 155, 71, 86, 1, 113, 92, 47, 20, 15, 192, 198, 34, 116, 145, 76, 179, 164, 95, 99, 82, 135, 127, 65, 196, 194, 11, 120, 200, 130, 40, 154, 14, 163, 64, 119, 39, 153, 13, 162, 161];

        assert_eq!(suffix_array.iter().map(|&e| e as i64).collect::<Vec<i64>>(), ans);
        assert_eq!(suffix_array_64, ans);
    }
}
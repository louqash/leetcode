use std::cmp;
use std::convert::TryInto;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut flags = [0; 255];
        let mut streak: i32 = 0;
        let mut max: i32 = 0;
        let mut last_clear: i32 = 0;
        let N: i32 = s.len() as i32;
        for (i, byte) in (0..N).zip(s.as_bytes().iter()) {
            let idx = *byte as usize;
            let last_pos = flags[idx];
            if last_pos >= last_clear {
                max = cmp::max(max, streak);
                streak = i - last_pos - 1;
                last_clear = last_pos;
            }
            flags[idx] = i;
            streak += 1;
        }
        cmp::max(max, streak)
    }
}


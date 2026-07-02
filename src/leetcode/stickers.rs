use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let sticker_vecs: Vec<[u8; 26]> = stickers.iter().map(|s| Self::word_to_vec(s)).collect();

        let target_vec = Self::word_to_vec(&target);

        let mut memo: HashMap<[u8; 26], Option<i32>> = HashMap::new();

        memo.insert([0u8; 26], Some(0));

        Self::dfs(target_vec, &sticker_vecs, &mut memo).unwrap_or(-1)
    }

    fn word_to_vec(word: &str) -> [u8; 26] {
        let mut count = [0u8; 26];

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            count[idx] += 1;
        }

        count
    }

    fn dfs(
        remain: [u8; 26],
        stickers: &[[u8; 26]],
        memo: &mut HashMap<[u8; 26], Option<i32>>,
    ) -> Option<i32> {
        if let Some(&cached) = memo.get(&remain) {
            return cached;
        }

        let first_needed = match remain.iter().position(|&x| x > 0) {
            Some(idx) => idx,
            None => return Some(0),
        };

        let mut best: Option<i32> = None;

        for sticker in stickers {
            if sticker[first_needed] == 0 {
                continue;
            }

            let mut next = remain;

            for i in 0..26 {
                next[i] = next[i].saturating_sub(sticker[i]);
            }

            if let Some(sub_ans) = Self::dfs(next, stickers, memo) {
                let candidate = sub_ans + 1;

                best = match best {
                    Some(old) => Some(old.min(candidate)),
                    None => Some(candidate),
                };
            }
        }

        memo.insert(remain, best);

        best
    }
}

pub fn test() {
    let input = vec![
        "with".to_string(),
        "example".to_string(),
        "science".to_string(),
    ];
    let target = "thehat".to_string();
    let result = Solution::min_stickers(input, target);
    println!("题目结果：{result:?}");
}

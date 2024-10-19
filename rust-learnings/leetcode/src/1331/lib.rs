// 1331 - https://leetcode.com/problems/rank-transform-of-an-array/solutions/5858173/easiest-approach-with-beats-94-01/
pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut mp: HashMap<i32, i32> = HashMap::new();
    let mut res: Vec<i32> = Vec::with_capacity(arr.len());
    let mut sorted: Vec<i32> = arr.clone();
    sorted.sort();
    sorted.dedup();

    for (i, n) in sorted.iter().enumerate() {
        mp.insert(*n, i as i32 + 1);
    }

    for n in arr {
        res.push(mp[&n]);
    }

    res
}

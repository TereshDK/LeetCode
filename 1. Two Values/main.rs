use std::collections::HashMap;

fn main()
{
    /// Two values test case.
    let values:Vec<i32> = vec![2, 7, 11, 15];
    let target:i32 = 9;
    let result = two_values(values, target);
    println!("{:?}", result); 
}

/// Return indicies of two values that add up to target.
pub fn two_values(values:Vec<i32>, target:i32) -> Vec<i32>
{
    let mut map:HashMap<i32, i32> = HashMap::new();

    /// O(n) search without O(n^2) brute force by using hash hash map
    /// to keep track of values already seen; lookup is O(1).
    for (idx, &val) in values.iter().enumerate()
    {
        let cmpl:i32 = target - val;
        if let Some(&cmpl_idx) = map.get(&cmpl)
        {
            return vec![cmpl_idx, idx as i32];
        }
        else
        {
            map.insert(val, idx as i32);
        }
    }

    vec![-1, -1]
}

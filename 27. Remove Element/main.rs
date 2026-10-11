fn main()
{
    /// Remove duplicates test case.
    let mut values:Vec<i32> = vec![0,1,2,2,3,0,4,2];
    let result = remove_element(&mut values, 2);
    println!("{}", result);
}

/// Remove all occurrences of val from array.
pub fn remove_element(values:&mut Vec<i32>, val:i32) -> i32
{
    let mut k:usize = 0;
    for pos in 0..values.len()
    {
        if values[pos] != val
        {
            values[k] = values[pos];
            k += 1;
        }
    }

    k as i32
}

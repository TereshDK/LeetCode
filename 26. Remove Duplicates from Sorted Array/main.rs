fn main()
{
    /// Remove duplicates from sorted array test case.
    let mut values:Vec<i32> = vec![1, 1, 2];
    let result = remove_duplicates(&mut values);
    println!("{}", result);
}

/// Remove duplicates from sorted array.
pub fn remove_duplicates(values:&mut Vec<i32>) -> i32
{
    if values.is_empty()
    {
        return 0;
    }

    let mut p_unique:usize = 1;
    for p_inspect in 1..values.len()
    {
        if values[p_inspect] != values[p_unique - 1]
        {
            values[p_unique] = values[p_inspect];
            p_unique += 1;
        }
    }

    p_unique as i32
}

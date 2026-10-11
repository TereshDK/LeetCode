fn main()
{
    /// Longest common prefix test case.
    let strs:Vec<String> = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let result = longest_common_prefix(strs);
    println!("{}", result); 
}

/// Find longest common prefix string amongst an array of strings.
pub fn longest_common_prefix(strs:Vec<String>) -> String
{
    let first:&str = &strs[0];

    for (pos, slice) in first.as_bytes().iter().enumerate()
    {
        for other in &strs[1..]
        {
            if pos == other.len() || other.as_bytes()[pos] != *slice
            {
                return first[0..pos].to_string();
            }
        }
    }

    first.to_string()
}

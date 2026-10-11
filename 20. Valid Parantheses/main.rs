fn main()
{
    /// Is paranthesis valid test case.
    let s:String = String::from("()[]{}");
    let result = is_valid(s);
    print!("{}", result);
}

/// Verify string has valid paranthesis and ordering.
pub fn is_valid(s:String) -> bool
{
    if s.len() <= 1
    {
        return false;
    }

    let mut stack:Vec<char> = Vec::new();

    for ch in s.chars()
    {
        match Some(ch)
        {
            Some('(') => stack.push(')'),
            Some('{') => stack.push('}'),
            Some('[') => stack.push(']'),
            _ =>
            {
                if stack.pop() != Some(ch)
                {
                    return false;
                }
            }
        }
    }

    match stack.len()
    {
        0 => return true,
        _ => return false,
    }
}

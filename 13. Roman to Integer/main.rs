fn main()
{
    /// Roman to int test case.
    let result = roman_to_int("MCMXCIV".to_string());
    println!("{}", result);
}

/// Helper function for converting Roman character to int.
pub fn roman_char_to_int(c:&char) -> i32
{
    match c
    {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

/// Convert a Roman character to an integer.
pub fn roman_to_int(s:String) -> i32
{
    let mut roman:String = s;
    let mut result:i32 = 0;
    let mut previous:i32 = 0;

    while let Some(c) = roman.pop()
    {
        let val = roman_char_to_int(&c);

        if val < previous
        {
            result -= val;
        }
        else
        {
            result += val;
            previous = val;
        }
    }

    result
}

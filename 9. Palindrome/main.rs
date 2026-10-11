fn main()
{
    /// Palindrome test case.
    let result = is_palindrome(121);
    println!("{}", result);
}

/// Check if an integer is a palindrome.
pub fn is_palindrome(x:i32) -> bool
{
    if x < 0
    {
        return false;
    }

    let copy:i32 = x;

    let mut x:i32 = x;
    let mut rev:i32 = 0;

    while x > 0
    {
        rev = rev * 10 + x % 10;
        x /= 10;
    }

    rev == copy
}

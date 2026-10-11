class Solution 
{
public:
    bool isPalindrome(int x) 
    {
        if (x < 0)
        {
            return false;
        }        

        int copy = x;
        long rev = 0;

        while (x > 0)
        {
            rev *= 10; 
            rev += x % 10;
            x /= 10;
        }

        return (rev == copy);
    }
};

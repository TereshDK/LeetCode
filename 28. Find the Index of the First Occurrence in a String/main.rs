#include <String>

class Solution 
{
public:
    int strStr(string haystack, string needle) 
    {
        int h = haystack.length();
        int n = needle.length();

        if (n > h)
        {
            return -1;
        }

        for (int p = 0; p <= (h - n); p++)
        {
            string sub = haystack.substr(p, n);
            if (sub == needle)
            {
                return p;
            }
        }

        return -1; 
    }
};

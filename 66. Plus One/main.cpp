#include <vector>

class Solution 
{
public:
    vector<int> plusOne(vector<int>& digits) 
    {
        for (int p = digits.size() - 1; p >= 0; p--)
        {
            if (digits[p] < 9)
            {
                digits[p]++;
                return digits;
            }
            digits[p] = 0;
        }
        digits.insert(digits.begin(), 1);

        return digits;
    }
};

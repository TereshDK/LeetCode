#include <Vector>

class Solution 
{
public:
    int searchInsert(vector<int>& values, int target) 
    {
        int n = values.size();

        if (target <= values[0]) 
        {
            return 0;
        }

        if (target > values[n - 1])
        {
            return n;
        }

        for (int p = 0; p < n; p++)
        {
            if ((values[p] <= target) && (target <= values[p + 1]))
            {
                return p + 1;
            }
        }

        return 0;    
    }
};

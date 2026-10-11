#include <vector>

class Solution 
{
public:
    int removeElement(vector<int>& values, int val) 
    {
        int k = 0; 
        for (int pos = 0; pos < values.size(); pos++)
        {
            if (values[pos] != val)
            {
                values[k] = values[pos];
                k++;
            }
        }

        return k;
    }
};

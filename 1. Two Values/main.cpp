#include <unordered_map>
#include <vector>

class Solution 
{
public:
    vector<int> twoValues(vector<int>& values, int target) 
    {
        unordered_map<int, int> map;
        for (int pos = 0; pos < values.size(); pos++)
        {
            int cmpl = target - values[pos];
            if (map.find(cmpl) != map.end())
            {
                return {map[cmpl], pos};
            }

            map[values[pos]] = pos;
        }

        return {};
    }
};

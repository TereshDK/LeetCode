#include <String>

class Solution 
{
public:
    int lengthOfLastWord(string s) 
    {
        bool notSpace = false;
        int len = 0;
        for (int p = s.length() - 1; p >= 0; p--)
        {
            if ((!notSpace) && (s.at(p) == ' '))
            {
                continue; 
            }
            else
            {
                if ((s.at(p) >= 'A' && s.at(p) <= 'Z') || (s.at(p) >= 'a' && s.at(p) <= 'z'))
                {
                    notSpace = true;
                    len++;
                }

                if (s.at(p) == ' ')
                {
                    return len;
                }
            }
        }

        return len;    
    }
};

/*
alternative soltuion

int len = 0;
for (int p = s.length() - 1; p >= 0; p--)
{
    if (s.at(p) != ' ')
    {
        len++;
    }
    else
    {
        if (len > 0)
        {
          return len;
        }
    }
}

return len;    
*/

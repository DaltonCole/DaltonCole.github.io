+++
title = "Eight Queens"
kattis_url = "https://open.kattis.com/problems/8queens"
difficulty = "3.7"
languages = ["C++"]
writeup = false
+++

## Code

#### C++

```cpp
#include <iostream>
using namespace std;

struct location
{
	int m_x;
	int m_y;
};

int main()
{
	location q[8];
	short index = 0;
	char input;
	bool worked = true;

	for(int i = 0; i < 8; i++)
	{
		for(int j = 0; j < 8; j++)
		{
			cin >> input;
			if(input == '*')
			{
				q[index].m_x = i;
				q[index].m_y = j;
				index++;
			}
		}
	}

	for(int i = 0; i < 7; i++)
	{
		for(int j = i+1; j < 8; j++)
		{
			if(q[i].m_x == q[j].m_x)
			{
				worked = false;
			}
			else if(q[i].m_y == q[j].m_y)
			{
				worked = false;
			}
			else if(static_cast<float>(((float)abs(q[i].m_y - q[j].m_y)/(float)abs(q[i].m_x - q[j].m_x))) == 1)
			{
				worked = false;
			}
		}
		if(worked == false)
		{
			break;
		}
	}

	if(worked == true)
	{
		cout << "valid" << endl;
	}
	else
	{
		cout << "invalid" << endl;
	}

	return 0;
}
```

+++
title = "Falling Apples"
kattis_url = "https://open.kattis.com/problems/apples"
difficulty = "3.2"
languages = ["C++"]
writeup = false
+++

## Code

#### C++

```cpp
#include <iostream>
#include <vector>
using namespace std;

int main() {
    int r = 0, c = 0;
    cin >> r >> c;
    vector<char> temp(r, '0');
    string input;
    char chara;
    
    vector<int> count(c, 0);

    // Input
    vector<vector<char> > map(c, temp); // 2D vector

    // map[column][row];
    for(int i = 0; i < r; i++) {
        cin >> input;
        for(int j = 0; j < c; j++) {
            //map[j][i] = input[j];
        	chara = input[j];
            if(chara == 'a') {
            	count[j]++;
            	map[j][i] = '.';
            } else if(chara == '#') {
            	map[j][i] = '#';
            	for(int k = 1; k < count[j] + 1; k++) {
            		map[j][i-k] = 'a';
            	}
            	count[j] = 0;
            } else {
            	map[j][i] = '.';
            }
        }
    }
    
    for(int i = 0; i < c; i++) {
        if(count[i] != 0) {
            for(int k = 0; k < count[i]; k++) {
                map[i][r - k - 1] = 'a';
            }
        }
    }
        
    for(int i = 0; i < r; i++) {
        for(int j = 0; j < c; j++) {
            cout << map[j][i];
        }
        cout << endl;
    }
    
    return 0;
}
```

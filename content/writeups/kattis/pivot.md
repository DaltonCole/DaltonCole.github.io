+++
title = "Pivot"
kattis_url = "https://open.kattis.com/problems/pivot"
difficulty = "3.3"
languages = ["C++", "Java"]
writeup = false
+++

## Code

#### C++

```cpp
#include <iostream>

using namespace std;

#define N 100000

int smaller[N];
int larger[N];
int container[N];

int main() {
	int number;
	int temp;

	cin >> number;

	for(int i = 0; i < number; i++) {
		cin >> temp;
		container[i] = temp;
	}

	temp = 0;
	for(int i = 0; i < number; i++) {
		temp = (temp > container[i]) ? temp : container[i];
		smaller[i] = temp;
	}
	temp = 999999;
	for(int i = number-1; i >= 0; i--) {
		temp = (temp < container[i]) ? temp : container[i];
		larger[i] = temp;
	}

	temp = 0;
	for(int i = 0; i < number; i++) {
		if(container[i] >= smaller[i] && container[i] <= larger[i]) {
			temp++;
		}
	}

	cout << temp << endl;

	return 0;
}
```

#### Java

```java
import java.util.Scanner;

public class Pivot {
	public static void main(String[] args) {
		Scanner s = new Scanner(System.in);

		int number = s.nextInt();

		int[] smaller = new int [number];
		int[] larger = new int [number];
		int[] container = new int [number];

		int temp;

		for(int i = 0; i < number; i++) {
			container[i] = s.nextInt();
		}

		temp = 0;
		for(int i = 0; i < number; i++) {
			if(temp < container[i]) {
				temp = container[i];
			}
			smaller[i] = temp;
		}

		temp = 999999;
		for(int i = number - 1; i >= 0; i--) {
			if(temp > container[i]) {
				temp = container[i];
			}
			larger[i] = temp;
		}

		temp = 0;
		for(int i = 0; i < number; i++) {
			if(smaller[i] <= container[i] && larger[i] >= container[i]) {
				temp++;
			}
		}

		System.out.println(temp);
	}
}
```

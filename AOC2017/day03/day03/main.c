#include <stdio.h>
#include <stdlib.h>
/*
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...
*/
#define INPUT 277678
void logic(int *i, int *j, int odd_num, int current) {
    if (current == INPUT) {
        return;
    }

    int steps = 0;
    while (steps < odd_num) {
        *j -= 1;
        current--;

        if (current == INPUT) {
            return;
        }
        steps++;
    }
    steps = 0;



    while (steps < odd_num) {
        *i -= 1;
        current--;

        if (current == INPUT) {
            return;
        }
        steps++;
    }

    steps = 0;

    while (steps < odd_num) {
        *j += 1;
        current--;

        if (current == INPUT) {
            return;
        }
        steps++;
    }
    steps = 0;



    while (steps < odd_num) {
        *i += 1;
        current--;

        if (current == INPUT) {
            return;
        }
        steps++;
    }
    steps = 0;

}
int main()
{

    int odds = 1, i = 0, j = 0;

    while (1) {
        if (odds * odds >= INPUT) {
            logic(&i, &j, odds, odds * odds);
            printf("answer is: %d", i + j);
            break;
        }
        i++, j++;
        odds += 2;

    }

    return 0;
}

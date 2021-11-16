#include <stdio.h>
#include <stdlib.h>
#define STEPS 303
#define B_LENGTH 50000000

int main()
{
    int cur_pos = 0, cur_length = 1, cur_num = 1, val_after_0 = 0;

    while (cur_num < (B_LENGTH + 1)) {
        int new_indx = (cur_pos + STEPS) % cur_length;

        if (new_indx == 0) {
            val_after_0 = cur_num;
        }

        if (cur_num == B_LENGTH) {
            printf("%d\n", val_after_0);
        }

        cur_length++;
        cur_num++;
        cur_pos = new_indx + 1;
    }


    return 0;
}

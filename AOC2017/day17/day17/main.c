#include <stdio.h>
#include <stdlib.h>
#define STEPS 303
#define B_LENGTH 2017

void insert_after(int indx, int value, int cur_length, int buffer[]) {

    int end_indx = cur_length, end_actual = cur_length - 1;

    while (end_indx > (indx + 1)) {
        buffer[end_indx--] = buffer[end_actual--];
    }
    buffer[end_indx] = value;

}

int main()
{
    int circular_buffer[B_LENGTH];

    for (int i = 0; i < B_LENGTH; i++) circular_buffer[i] = 0;

    int cur_pos = 0, cur_length = 1, cur_num = 1;

    while (cur_num < (B_LENGTH + 1)) {
        int new_indx = (cur_pos + STEPS) % cur_length;

        if (cur_num == B_LENGTH) {
            printf("%d\n", circular_buffer[new_indx + 1]);
        }
        insert_after(new_indx, cur_num, cur_length, circular_buffer);
        cur_length++;
        cur_num++;
        cur_pos = new_indx + 1;
    }


    return 0;
}

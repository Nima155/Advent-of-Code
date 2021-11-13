#include <stdio.h>
#include <stdlib.h>
#define LIST_LEN 16
#define NUMBERS_LEN 256
// 94,84,0,79,2,27,81,1,123,93,218,23,103,255,254,243
void reverse_in_place(int buffer[], int start, int slice_size) {

    int end_point = (start + slice_size) - 1;



    for (int j = start; j <  (start + (start + slice_size)) / 2; j++) {
        int wrapped_end_indx = end_point % NUMBERS_LEN,
            wrapped_start_indx = j % NUMBERS_LEN;
        int temp = buffer[wrapped_end_indx];

        buffer[wrapped_end_indx] = buffer[wrapped_start_indx];

        buffer[wrapped_start_indx] = temp;
        end_point--;
    }
}

int main()
{
    const int arr[] = {94, 84, 0, 79, 2, 27, 81, 1, 123, 93, 218, 23, 103, 255, 254, 243};
    // 8742 too low
    int numbers[NUMBERS_LEN];
    for (int i = 0; i < NUMBERS_LEN; i++) numbers[i] = i;

    int skip_size = 0, cur_pos = 0;

    for (int j = 0; j < LIST_LEN; j++) {
        reverse_in_place(numbers, cur_pos, arr[j]);
        cur_pos += arr[j];
        cur_pos += skip_size++;

    }

    printf("%d\n", numbers[0] * numbers[1]);


    return 0;
}

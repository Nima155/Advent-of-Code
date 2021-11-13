#include <stdio.h>
#include <stdlib.h>
#define NUMBERS_LEN 256
#define MAX_CYCLE 64
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

static const int extension[] = { 17, 31, 73, 47, 23 };

void create_hash(int numbers[]) {

    int xored[16];
    int st = 16, xor_indx = 0;

    while (st <= 256) {
        int starting = numbers[st - 16];

        for (int i = st - 15; i < st; i++) {
            starting ^= numbers[i];
        }
        xored[xor_indx++] = starting;

        st += 16;
    }

    printf("ans: ");
    for (int i = 0; i < 16; i++) {
        printf("%02x", xored[i]);
    }
}

int main()
{
    FILE *file_handle = fopen("input.txt", "r");

    if (file_handle == NULL) {
        return 1;
    }

    int arr[500];
    int indx = 0, cur_car = 0;

    while ((cur_car = fgetc(file_handle)) != '\n' && cur_car != EOF) {
        arr[indx++] = cur_car;
    }

    for (int i = 0; i < 5; i++) arr[indx++] = extension[i];






    // 8742 too low
    int numbers[NUMBERS_LEN];
    for (int i = 0; i < NUMBERS_LEN; i++) numbers[i] = i;

    int skip_size = 0, cur_pos = 0, c = 0;

    while (c < MAX_CYCLE) {
        for (int j = 0; j < indx; j++) {
            reverse_in_place(numbers, cur_pos, arr[j]);
            cur_pos += arr[j];
            cur_pos += skip_size++;
        }
        c++;
    }

    create_hash(numbers);

    fclose(file_handle);
    return 0;
}

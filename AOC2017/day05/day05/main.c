#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#define LENGTH 1074

void read_file_into_buffer(int buffer[], FILE *file_handle) {

    int c = 0, i = 0, nega = 0, cur_num = 0;

    while ((c = fgetc(file_handle)) != EOF) {
        if (c == '-') {
            nega = 1;
        }

        if (isdigit(c)) {
            cur_num *= 10;
            cur_num += (c - '0');
        } else if (c == '\n'){

            buffer[i++] = nega ? -cur_num: cur_num;
            cur_num = 0;
            nega = 0;
        }


    }

}

void run_the_instructions(int buffer[]) {
    int steps = 0, i = 0;

    while (i < LENGTH && i >= 0) {
        int tmp = i, offset_was_over = buffer[i] >= 3;
        i += buffer[i];
        // buffer[tmp]++; for part 1
        buffer[tmp] += offset_was_over ? -1: 1; // for part 2
        steps++;
    }
    printf("steps: %d\n", steps);

}

int main()
{
    FILE *f = fopen("input.txt", "r");

    if (f == NULL) {
        return 1;
    }

    int buffer[LENGTH] = { 0 };

    read_file_into_buffer(buffer, f);

    run_the_instructions(buffer);

    fclose(f);


}

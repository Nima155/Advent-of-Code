#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <limits.h>

#define ROW_LENGTH 30

int *input_reader(FILE *file_handle) {
    int *buffer = malloc(ROW_LENGTH * sizeof(int));

    for (int i = 0; i < ROW_LENGTH; i++) {
        buffer[i] = 0;
    }

    int cur = 0, cur_num = 0, j = 0, i = 0;

    while ((cur = getc(file_handle)) != '\n' && cur != EOF) {

        if (isdigit(cur)) {
            cur_num *= 10;
            cur_num += (cur - '0');
        } else if (cur_num != 0) {
            buffer[j] = cur_num;
            cur_num = 0;
            j++;
        }
        i++;
    }
    buffer[j] = cur_num;
    return buffer;
}

int diff_finder(int row[]) {
    int min = INT_MAX, max = INT_MIN;

    for (int i = 0; i < ROW_LENGTH; i++) {
        if (row[i] != 0) {

            min = min > row[i] ? row[i]: min;
            max = max < row[i] ? row[i]: max;
        }
    }

    return  min == INT_MAX ? 0: max - min;

}

int main()
{
    FILE *f = fopen("input.txt", "r");

    if (f == NULL) {
        return 1;
    }
    int ans = 0;
    while (1) {
        int *temp = input_reader(f);
        int tempz = diff_finder(temp);

        free(temp);

        if (tempz == 0) {
            break;
        }
        ans += tempz;

    }
    printf("%d", ans);

    fclose(f);


    return 0;
}



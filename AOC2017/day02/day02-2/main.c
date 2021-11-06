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

int even_division_finder(int row[]) {

    for (int i = 0; i < ROW_LENGTH; i++) {
        for (int j = 0; j < ROW_LENGTH; j++) {
            if (row[i] != 0 && row[j] != 0 && j != i && row[i] % row[j] == 0) {
                return row[i] / row[j];
            }
        }

    }

    return 0;

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
        int tempz = even_division_finder(temp);

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



#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <limits.h>

#define ROW_LENGTH 30

int static_buffer[ROW_LENGTH] = { 0 };

void input_reader(FILE *file_handle) {

    for (int i = 0; i < ROW_LENGTH; i++) {
        static_buffer[i] = 0;
    }

    int cur = 0, cur_num = 0, j = 0, i = 0;

    while ((cur = getc(file_handle)) != '\n' && cur != EOF) {

        if (isdigit(cur)) {
            cur_num *= 10;
            cur_num += (cur - '0');
        } else if (cur_num != 0) {
            static_buffer[j] = cur_num;
            cur_num = 0;
            j++;
        }
        i++;
    }
    static_buffer[j] = cur_num;

}

int even_division_finder() {

    for (int i = 0; i < ROW_LENGTH; i++) {
        for (int j = 0; j < ROW_LENGTH; j++) {
            if (static_buffer[i] != 0 && static_buffer[j] != 0 && j != i && static_buffer[i] % static_buffer[j] == 0) {
                return static_buffer[i] / static_buffer[j];
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

        input_reader(f);

        int tempz = even_division_finder();

        if (tempz == 0) {
            break;
        }

        ans += tempz;

    }
    printf("%d", ans);

    fclose(f);


    return 0;
}



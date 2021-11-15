#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#define STARTER_STRING "hfdlxzhv"
#define NUMBERS_LEN 256
#define MAX_CYCLE 64
#define GRID_SIZE 16384
void reverse_in_place_normal(char *buffer) {
    int len = strlen(buffer);
    int end = len - 1;
    for (int i = 0; i < len / 2; i++) {
        char tmp = buffer[end];
        buffer[end--] = buffer[i];
        buffer[i] = tmp;
    }
}

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

char *to_binary(int num) {
    char *bin_rep = malloc(30);
    int indx = 0, num_cop = num;

    if (bin_rep == NULL) {
        return NULL;
    }

    bin_rep[0] = '\0';

    do {
        bin_rep[indx++] = (num % 2) + '0';
        num /= 2;
    } while (num > 0);

    while (indx < 4) {
        bin_rep[indx++] = '0';
    }

    bin_rep[indx] = '\0';

    reverse_in_place_normal(bin_rep);



    return bin_rep;

}

static const int extension[] = { 17, 31, 73, 47, 23 };

char *create_hash(int numbers[]) {

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

    char *hexadecimal_rep = malloc(100);

    if (hexadecimal_rep == NULL) {
        return NULL;
    }

    *hexadecimal_rep = '\0';

    for (int i = 0; i < 16; i++) {
        char temp[10];

        sprintf(temp, "%02x", xored[i]);

        strcat(hexadecimal_rep, temp);


    }
    return hexadecimal_rep;

}

char *hexa_to_bin(char *hexadecimal) {
    char *grid = malloc(128 + 1);
    if (grid == NULL) {
        return NULL;
    }
    *grid = '\0';

    int len = strlen(hexadecimal), ans = 0;

    for (int i = 0; i < len; i++) {
        char current_car = hexadecimal[i];

        int int_rep_of_car = isdigit(current_car)
                             ? current_car - '0': (current_car - 'a') + 10;


        char *bin_rep_2 = to_binary(int_rep_of_car);

        strcat(grid, bin_rep_2);
        free(bin_rep_2);

    }
    return grid;




}

void dfs(int visited[], const char *grid, int cur_indx) {
    if (visited[cur_indx] != 0) {
        return;
    }


    visited[cur_indx] = 1;

    int l = cur_indx - 1, r = cur_indx + 1,
            u = cur_indx - 128, d = cur_indx + 128;

    if (l >= 0 && (cur_indx % 128) != 0 && grid[l] == '1') {
        dfs(visited, grid, l);
    }
    if (r < GRID_SIZE && ((cur_indx + 1) % 128) != 0 && grid[r] == '1') {
        dfs(visited, grid, r);
    }
    if (u >= 0 && grid[u] == '1') {
        dfs(visited, grid, u);
    }
    if (d < GRID_SIZE && grid[d] == '1') {
        dfs(visited, grid, d);
    }

}

int main()
{
    FILE *file_handle = fopen("input.txt", "r");
    char grid[GRID_SIZE + 1];
    if (file_handle == NULL) {
        return 1;
    }
    *grid = '\0';

    int arr[500];
    int indx = 0, cur_car = 0;

    while ((cur_car = fgetc(file_handle)) != '\n' && cur_car != EOF) {
        arr[indx++] = cur_car;
    }




    for (int j = 0; j < 128; j++) {
        int starting = indx;
        char buff[10];
        int i = 0;
        sprintf(buff, "-%d", j);


        for (; i < strlen(buff); i++) arr[starting++] = buff[i];
        for (i = 0; i < 5; i++) arr[starting++] = extension[i];

        int numbers[NUMBERS_LEN];

        for (i = 0; i < NUMBERS_LEN; i++) numbers[i] = i;

        int skip_size = 0, cur_pos = 0, c = 0;

        while (c < MAX_CYCLE) {
            for (int m = 0; m < starting; m++) {
                reverse_in_place(numbers, cur_pos, arr[m]);
                cur_pos += arr[m];
                cur_pos += skip_size++;
            }
            c++;
        }

        char *hexa_rep = create_hash(numbers);

        char *row = hexa_to_bin(hexa_rep);

        strcat(grid, row);

        free(hexa_rep);
        free(row);

    }

    // 1087 too low

    int visited[GRID_SIZE];

    for (int i = 0; i < GRID_SIZE; i++) visited[i] = 0;

    int ans = 0;

    for (int i = 0; i < GRID_SIZE; i++) {
        if (!visited[i] && grid[i] == '1') {
            dfs(visited, grid, i);
            ans++;
        }
    }

    printf("ans: %d\n", ans);

    fclose(file_handle);
    return 0;
}

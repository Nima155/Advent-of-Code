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
#define MAX_X 100000
#define MAX_Y 1000

int moves[8][2] = { {0, 1}, {1, 0}, {0, -1}, {-1, 0}, {1, 1}, {-1, -1}, {1, -1}, {-1, 1} };

int sum_of_neighbors(int i, int j, int *grid) {
    int sums = 0;
    for (int m = 0; m < 8; m++) {
        int y = moves[m][0] + i, x = moves[m][1] + j;

        sums += grid[y * MAX_X + x];
    }
    return sums;
}

int logic(int i, int j, int odd_num, int *grid) {

    int steps = 0;
    while (steps < (odd_num - 2)) {
        grid[i * MAX_X + j] = sum_of_neighbors(i, j, grid);

        if (grid[i * MAX_X + j] > INPUT) {
            return grid[i * MAX_X + j];
        }
        i -= 1;
        steps++;
    }

    steps = 0;

    while (steps < odd_num) {
        grid[i * MAX_X + j] = sum_of_neighbors(i, j, grid);

        if (grid[i * MAX_X + j] > INPUT) {
            return grid[i * MAX_X + j];
        }
        j -= 1;
        steps++;
    }
    j += 1;
    steps = 0;

    while (steps < odd_num) {
        grid[i * MAX_X + j] = sum_of_neighbors(i, j, grid);

        if (grid[i * MAX_X + j] > INPUT) {
            return grid[i * MAX_X + j];
        }
        i += 1;
        steps++;
    }
    i -= 1;
    steps = 0;



    while (steps < odd_num) {
        grid[i * MAX_X + j] = sum_of_neighbors(i, j, grid);

        if (grid[i * MAX_X + j] > INPUT) {
            return grid[i * MAX_X + j];
        }
        j += 1;
        steps++;
    }

    return 0;

}
int main()
{

    int odds = 3, i = 500, j = 50000;

    int *grid = malloc(sizeof(int) * MAX_Y * MAX_X);

    for (int v = 0; v < MAX_X * MAX_Y; v++) {
            grid[v] = 0;
    }

    grid[i * MAX_X + j] = 1;
    j++;

    while (1) {
        int tmp = logic(i, j, odds, grid);

        if (tmp != 0) {
            printf("%d", tmp);
            break;
        }
        i++;
        j++;
        odds += 2;
    }
    free(grid);
    return 0;
}

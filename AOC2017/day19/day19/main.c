#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <stdbool.h>
#include <string.h>
#define HEIGHT 200
#define HEIGHT_ROW ((HEIGHT) * (HEIGHT))


int moves[4][2] = {{0, 1}, {1, 0 }, {0, -1}, {-1, 0}};

bool is_valid(const char c) {
    return c == '|' || c == '+' || c == '-' || isalpha(c);
}

void play_the_game(const char **grid, const int x_pos) {
    int visited[HEIGHT_ROW];
    for (int i = 0; i < HEIGHT_ROW; i++) visited[i] = 0;

    int m_x = 0, m_y = 1, ny = 0, nx = x_pos, steps = 1;
    // 16262 too low
    printf("letters: ");
    while (1) {
        while (ny >= 0 && nx >= 0 && ny < HEIGHT && nx < strlen(grid[ny]) && is_valid(grid[ny][nx])) {
            if (isalpha( grid[ny][nx] ) && !visited[ny * HEIGHT + nx]) {
                printf("%c", grid[ny][nx]);
            }
            visited[ny * HEIGHT + nx] = 1;
            ny += m_y, nx += m_x, steps++;
        }

        ny -= m_y;
        nx -= m_x;
        steps--;
        bool move_available = false;
        for (int j = 0; j < 4; j++) {
            int nyyy = ny + moves[j][0], nxxx = nx +  moves[j][1];
            if (nyyy >= 0 && nyyy < HEIGHT && nxxx >= 0 && nxxx < strlen(grid[nyyy]) && is_valid(grid[nyyy][nxxx]) && !visited[nyyy * HEIGHT + nxxx]) {
                m_x = moves[j][1];
                m_y = moves[j][0];
                move_available = true;
                break;
            }
        }
        if (!move_available) break;

    }
    printf("\nsteps: %d\n", steps);
}

int main()
{
    FILE *f = fopen("input.txt", "r");
    if (f == NULL) {
        return 1;
    }

    char **grid = malloc(sizeof(char *) * 200);

    int indx = 0, c = 0, inner_indx = 0, starting_x = 0;



    char *row = malloc(HEIGHT + 1);

    while (1) {
        c = fgetc(f);
        if (c == '\n') {

            row[inner_indx] = '\0';
            grid[indx++] = row;
            if (indx == HEIGHT) break;
            row = malloc(HEIGHT + 1);
            inner_indx = 0;
        } else {
            if (c == '|' && starting_x == 0) {
                starting_x = inner_indx;
            }
            row[inner_indx++] = c;
        }
    }

    play_the_game((const char **)grid, starting_x);
    for (indx = 0; indx < HEIGHT; indx++) {
        free(grid[indx]);
    }
    free(grid);


    fclose(f);

    return 0;
}

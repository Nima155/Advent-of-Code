#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <math.h>
#include <limits.h>
#define DIMENSION 3
#define MAX_CYCLES 1000000
#define TOTAL_POINTS 1000
#define INIT_TO_ZERO(array) \
    for (int i = 0; i < DIMENSION; i++) { \
        for (int j = 0; j < DIMENSION; j++) { \
            (array[i][j]) = 0; \
        } \
    }
typedef struct Point Point;
typedef struct Object Object;

struct Point {
    int y, x, z;
};

struct Object {
    Point velocity, acceleration, positions;

};

void simulate_the_process(Object *objects) {

    int point_number = 0;

    while (1) {
        int at_least_one_getting_closer = 0, min_dist = INT_MAX;
        for (int i = 0; i < TOTAL_POINTS; i++) {
            objects[i].velocity.x += objects[i].acceleration.x;
            objects[i].velocity.y += objects[i].acceleration.y;
            objects[i].velocity.z += objects[i].acceleration.z;
            int before = abs(objects[i].positions.z) + abs(objects[i].positions.y) + abs(objects[i].positions.x);
            objects[i].positions.x += objects[i].velocity.x;
            objects[i].positions.y += objects[i].velocity.y;
            objects[i].positions.z += objects[i].velocity.z;
            int dist = abs(objects[i].positions.z) + abs(objects[i].positions.y) + abs(objects[i].positions.x);

            at_least_one_getting_closer |= dist < before;


            if (dist <= min_dist) {
                min_dist = dist;
                point_number = i;
            }
        }

        if (!at_least_one_getting_closer) {
            break;
        }

    }
    printf("ans: %d\n", point_number);

}

int main()
{
    FILE *f = fopen("input.txt", "r");
    if (f == NULL) {
        return 1;
    }
    int oj_indx = 0;
    Object *objects = malloc(sizeof(Object) * TOTAL_POINTS);


    while (1) {
        int c = 0, cur_stat_indx = 0, inner_indx = 0, is_nega = 0;
        int tempo[3][3];
        INIT_TO_ZERO(tempo);

        while ((c = fgetc(f)) != EOF && c != '\n') {
            if (isdigit(c)) {
                tempo[cur_stat_indx][inner_indx] *= 10;
                tempo[cur_stat_indx][inner_indx] += (c - '0');
            }
            else if (c == ',') {
                if (is_nega) {
                    tempo[cur_stat_indx][inner_indx] = -tempo[cur_stat_indx][inner_indx];
                }
                if (inner_indx + 1 != 3) {
                    inner_indx++;

                } else {
                    cur_stat_indx++;
                    inner_indx = 0;
                }
                is_nega = 0;
            }
            else if (c == '-') {
                is_nega = 1;
            }
        }

        if (is_nega) {
            tempo[cur_stat_indx][inner_indx] = -tempo[cur_stat_indx][inner_indx];
        }

        if (c == EOF) {
            break;
        }

        Object obj = {.positions = { tempo[0][0], tempo[0][1], tempo[0][2]},
                          .velocity =   {tempo[1][0], tempo[1][1], tempo[1][2] },
                           .acceleration =  {tempo[2][0], tempo[2][1], tempo[2][2] }};
        objects[oj_indx++] = obj;
    }
    simulate_the_process(objects);
    free(objects);
    fclose(f);
    return 0;
}

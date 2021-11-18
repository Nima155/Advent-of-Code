#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <math.h>
#include <limits.h>
#include "../../uthash-master/src/uthash.h"
#define DIMENSION 3
#define MAX_CYCLES 1000
#define TOTAL_POINTS 1000
#define INIT_TO_ZERO(array) \
    for (int i = 0; i < DIMENSION; i++) { \
        for (int j = 0; j < DIMENSION; j++) { \
            (array[i][j]) = 0; \
        } \
    }
typedef struct Point Point;
typedef struct Object Object;
typedef struct Node Node;

struct Node {
    const char *str_position;
    int indx;
    UT_hash_handle hh;
};

struct Point {
    int y, x, z;
};

struct Object {
    Point velocity, acceleration, positions;

};

void delete_all(Node **map) {
  Node *cur_node, *tmp;

  HASH_ITER(hh, *map, cur_node, tmp) {
    HASH_DEL(*map, cur_node);  /* delete; users advances to next */
    free(cur_node->str_position);
    free(cur_node);             /* optional- if you want to free  */
  }
}

void simulate_the_process(Object *objects) {

    int cycles = 0;


    int dead_points[TOTAL_POINTS];
    for (int i = 0; i < TOTAL_POINTS; i++) dead_points[i] = 0;

    Node *node_map = NULL;
    while (cycles < MAX_CYCLES) {

        for (int i = 0; i < TOTAL_POINTS; i++) {
            if (!dead_points[i]) {
                objects[i].velocity.x += objects[i].acceleration.x;
                objects[i].velocity.y += objects[i].acceleration.y;
                objects[i].velocity.z += objects[i].acceleration.z;

                objects[i].positions.x += objects[i].velocity.x;
                objects[i].positions.z += objects[i].velocity.z;
                objects[i].positions.y += objects[i].velocity.y;

                Node *node = NULL;

                char dummy_str[50];
                sprintf(dummy_str, "%d, %d, %d",
                        objects[i].positions.z,
                         objects[i].positions.x, objects[i].positions.y);

                HASH_FIND_STR(node_map, dummy_str, node);

                if (node) {
                    dead_points[node->indx] = 1;
                    dead_points[i] = 1;
                }

                else {
                    node = malloc(sizeof(Node));
                    if (node != NULL) {
                        node->indx = i;
                        node->str_position = malloc(50);
                        strcpy(node->str_position, dummy_str);

                        HASH_ADD_KEYPTR(hh, node_map,
                                        node->str_position,
                                        strlen(node->str_position),
                                        node);
                    }

                }
            }

        }
        delete_all(&node_map);
        cycles++;
    }

    int count = 0;
    for (int i = 0; i < TOTAL_POINTS; i++) {
        count += !dead_points[i];
    }
    printf("count: %d\n", count);
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

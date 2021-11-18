#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "../../uthash-master/src/uthash.h"
#define GRID_SIZE 25
#define MAX_CYCLE 10000
typedef struct Node Node;

struct Node {
    char *position;
    bool is_infected;
    UT_hash_handle hh;
};

Node *node_map = NULL;

void delete_all() {
  Node *cur_node, *tmp;

  HASH_ITER(hh, node_map, cur_node, tmp) {
    HASH_DEL(node_map, cur_node);  /* delete; users advances to next */
    free(cur_node->position);
    free(cur_node);             /* optional- if you want to free  */
  }
}



int main()
{
    FILE *f = fopen("input.txt", "r");
    if (f == NULL) {
        return 1;
    }

    int i = 0, j = 0, c = 0;

    while ((c = fgetc(f)) != EOF) {
        if (c == '\n') {
            i++;
            j = 0;
        } else {
            Node *node = malloc(sizeof(Node));
            if (node != NULL) {
                node->is_infected = c == '#';
                node->position = malloc(50);
                if (node->position) {
                    sprintf(node->position, "%d %d", i, j);
                    HASH_ADD_KEYPTR(hh, node_map, node->position, strlen(node->position), node);
                    j++;
                }
            }
        }
    }


    spread_the_virus();
    delete_all();
    fclose(f);
    // free the node_map elements
    return 0;
}

int moves[4][2] = {{-1, 0}, {0, 1},{1, 0},{0, -1}};

void spread_the_virus() {
    int x = GRID_SIZE / 2, y = GRID_SIZE / 2, cycles = 0, direction_indx = 0, infected_by_us = 0;

    while (cycles < MAX_CYCLE) {
        Node *cur_node = NULL;

        char key[50];
        sprintf(key, "%d %d", y, x);
        HASH_FIND_STR(node_map, key , cur_node);

        if (cur_node == NULL) {
            cur_node = malloc(sizeof(Node));
            cur_node->is_infected = false;
            cur_node->position = malloc(50);
            strcpy(cur_node->position, key);
            HASH_ADD_KEYPTR(hh, node_map, cur_node->position, strlen(cur_node->position), cur_node);
        }


        if (cur_node->is_infected) {
            direction_indx = (direction_indx + 1) % 4;
        }
        else {
            direction_indx = ((direction_indx - 1) >= 0 ? direction_indx - 1: 3);
        }

        cur_node->is_infected = !cur_node->is_infected;
        infected_by_us += cur_node->is_infected;
        y += moves[direction_indx][0];
        x += moves[direction_indx][1];
        cycles++;
    }
    printf("%d\n", infected_by_us);
}

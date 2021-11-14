#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include "../../uthash-master/src/uthash.h"

typedef struct Node Node;

struct Node {
    int depth;
    int current_cursor_position;
    int id;
    char direction;
    UT_hash_handle hh;
};

Node *nodes = NULL; // part of hashmap initialization

void perform_cycle() {
    for (Node *cur_node = nodes; cur_node != NULL; cur_node = cur_node->hh.next) {
        switch (cur_node->direction) {
        case 'v':
            if (cur_node->current_cursor_position + 1 < cur_node->depth) {
                cur_node->current_cursor_position++;
            } else {
                cur_node->direction = '^';
                cur_node->current_cursor_position--;
            }
            break;
        case '^':
            if (cur_node->current_cursor_position - 1 >= 0) {
                cur_node->current_cursor_position--;
            } else {
                cur_node->direction = 'v';
                cur_node->current_cursor_position++;
            }
            break;
        }
    }
}

int let_the_rider_ride(int final_obstacle) {
    int rider_pos = 0, ans = 0;

    while (rider_pos <= final_obstacle) {
        Node *found_node = NULL;
        HASH_FIND_INT(nodes, &rider_pos, found_node);


        if (found_node != NULL && found_node->current_cursor_position == 0) {
            ans += (found_node->depth * found_node->id);
        }

        perform_cycle();
        rider_pos++;

    }
    return ans;
}

void delete_all() {
  Node *cur_node, *tmp;

  HASH_ITER(hh, nodes, cur_node, tmp) {
    HASH_DEL(nodes, cur_node);  /* delete; users advances to next */
    free(cur_node);             /* optional- if you want to free  */
  }
}

int main()
{
    FILE *f_handle = fopen("input.txt", "r");

    if (f_handle == NULL) {
        return 1;
    }

    int c = 0, cur_indx = 0, cur_depth = 0, after_colon = 0, maximum_indx = 0;


    // reading the file and populating the hashmap.
    while ((c = fgetc(f_handle)) != EOF) {
        if (isdigit(c)) {
            if (!after_colon) {
                cur_indx *= 10;
                cur_indx += (c - '0');
            }
            else {
                cur_depth *= 10;
                cur_depth += (c - '0');
            }
        }

        else if (c == '\n') {

            maximum_indx = cur_indx > maximum_indx ? cur_indx: maximum_indx;

            Node *node = malloc(sizeof(Node));
            if (node != NULL) {
                node->current_cursor_position = 0;
                node->depth = cur_depth;
                node->id = cur_indx;
                node->direction = 'v';
                HASH_ADD_INT(nodes, id, node);
            }
            cur_indx = cur_depth = after_colon = 0;
        }
        else if (c == ':') {
            after_colon = 1;
        }
    }

    printf("ans: %d\n", let_the_rider_ride(maximum_indx));
    delete_all();





    fclose(f_handle);
}

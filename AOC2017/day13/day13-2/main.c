#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include "../../uthash-master/src/uthash.h"

typedef struct Node Node;

struct Node {
    int depth;
    int id;
    UT_hash_handle hh;
};




int let_the_rider_ride(int final_obstacle, int delay, Node **nodes) {
    int rider_pos = 0, ans = 0, layer = delay;



    while (rider_pos <= final_obstacle) {
        Node *found_node = NULL;
        HASH_FIND_INT(*nodes, &rider_pos, found_node);

        if (found_node != NULL) {
            int in_sight = (delay % ((found_node->depth - 2) + found_node->depth)) == 0;

            if (in_sight) {
                return 1;
            }

        }
        rider_pos++;
        delay++;

    }

    return ans;
}

void delete_all(Node **nodes) {

  Node *cur_node, *tmp;

  HASH_ITER(hh, *nodes, cur_node, tmp) {
    HASH_DEL(*nodes, cur_node);

    free(cur_node);
  }
}

Node *clone_hasht(Node **nodes) {
    Node *nodez = NULL;

    for (Node *cur = *nodes; cur != NULL; cur = cur->hh.next) {
        Node *temp = malloc(sizeof(Node));
        if (temp != NULL) {

            temp->depth = cur->depth;
            temp->id = cur->id;

            HASH_ADD_INT(nodez, id, temp);
        }
    }

    return nodez;

}

int main()
{
    FILE *f_handle = fopen("input.txt", "r");
    Node *nodes = NULL; // part of hashmap initialization
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
                node->depth = cur_depth;
                node->id = cur_indx;
                HASH_ADD_INT(nodes, id, node);
            }
            cur_indx = cur_depth = after_colon = 0;
        }
        else if (c == ':') {
            after_colon = 1;
        }
    }





    for (int i = 0;;i += 2) {

        Node *map_clone = clone_hasht(&nodes);

        int temp = let_the_rider_ride(maximum_indx, i, &map_clone);

        delete_all(&map_clone);

        if (!temp) {
            printf("ans: %d\n", i);
            break;
        }


    }

    delete_all(&nodes);



    fclose(f_handle);
}

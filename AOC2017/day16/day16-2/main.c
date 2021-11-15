#include <stdio.h>
#include <stdlib.h>
#include "../../uthash-master/src/uthash.h"
#define PROGRAM_LENGTH 17
#define TOTAL_INSTRUCTIONS 10000
#define TOTAL_CYCLES 1000000000
// s1, a spin of size 1: eabcd.
// x3/4, swapping the last two programs: eabdc.
// pe/b, swapping programs e and b: baedc.
typedef struct Node Node;

struct Node {
    char *buffer;
    UT_hash_handle hh;
};


void delete_all(Node **map) {
  struct Node *node, *tmp;

  HASH_ITER(hh, *map, node, tmp) {
    HASH_DEL(*map, node);
    free(node->buffer); /* delete; users advances to next */
    free(node);             /* optional- if you want to free  */
  }
}

void pchange(char instruction[], char program[]) {
    char a = instruction[1], b = instruction[3];
    int indx_a = 0, indx_b = 0;
    for (int i = 0; i < PROGRAM_LENGTH; i++) {
        if (program[i] == a) {
            indx_a = i;
        } else if (program[i] == b) {
            indx_b = i;
        }
    }
    char temp = program[indx_a];

    program[indx_a] = program[indx_b];
    program[indx_b] = temp;



}

void xchange(char instruction[], char program[]) {
    int a = 0, b = 0, i = 1, after_div = 0;
    while (instruction[i] != '\0' && instruction[i] != '\n') {
        if (instruction[i] == '/') {
            after_div = 1;
        }
        else if (after_div) {
            b *= 10;
            b += (instruction[i] - '0');
        } else {
            a *= 10;
            a += (instruction[i] - '0');
        }
        i++;
    }

    char temp = program[b];

    program[b] = program[a];
    program[a] = temp;

}

void spin(char instruction[], char program[]) {
    int spins = 0, i = 1;

    while (instruction[i] != '\0') {
        spins *= 10;
        spins += (instruction[i] - '0');
        i++;
    }

    int from_indx = PROGRAM_LENGTH - (1 + spins), end_c_indx = 0,
                    from_before = PROGRAM_LENGTH - (1 + spins), start_c_indx = 0;

    char end_clone[25];

    while (from_indx < PROGRAM_LENGTH - 1) {
        end_clone[end_c_indx] = program[from_indx];
        end_c_indx++, from_indx++;
    }

    while (start_c_indx < from_before) {
        end_clone[end_c_indx] = program[start_c_indx];
        end_c_indx++, start_c_indx++;
    }
    end_clone[end_c_indx] = '\0';

    start_c_indx = 0;
    while (start_c_indx < PROGRAM_LENGTH) {
        program[start_c_indx] = end_clone[start_c_indx];
        start_c_indx++;
    }


}


int main()
{
    FILE *f = fopen("input.txt", "r");
    if (f == NULL) {
        return 1;
    }
    Node *node_map = NULL;

    char initial_program[PROGRAM_LENGTH] = "abcdefghijklmnop";

    char current_instruction[20];

    char *buffer[TOTAL_INSTRUCTIONS];
    int c = 0, cur_i_index = 0, buffer_indx = 0;

    while (1) {
        c = fgetc(f);
        if (c == ',' || c == EOF) {
            current_instruction[cur_i_index] = '\0';
            buffer[buffer_indx] = malloc(8);
            if (buffer[buffer_indx] != NULL) {
                strcpy(buffer[buffer_indx], current_instruction);
            }
            cur_i_index = 0, buffer_indx++;
            if (c == EOF) {
                break;
            }
        } else {
            current_instruction[cur_i_index++] = c;
        }
    }

    int cycle_number = 0, s_c_i = 0;
    char *saved_cycles[100];
    while (1) {
        Node *node = NULL;

        for (int j = 0; j < TOTAL_INSTRUCTIONS; j++) {
            switch (buffer[j][0]) {
               case 's':
                   spin(buffer[j], initial_program);
                   break;
               case 'x':
                   xchange(buffer[j], initial_program);
                   break;
               case 'p':
                   pchange(buffer[j], initial_program);
                   break;
           }
        }

        cycle_number++; // cycle starts after 24 turns, and that's what we exploit!

        saved_cycles[s_c_i] = malloc(PROGRAM_LENGTH);

        if (saved_cycles[s_c_i]) {
            strcpy(saved_cycles[s_c_i], initial_program);
        }

        s_c_i++;
        HASH_FIND_STR(node_map, initial_program, node);

        if (node == NULL) {
            node = malloc(sizeof(Node));
            node->buffer = malloc(PROGRAM_LENGTH);

            if (node->buffer) {
                strcpy(node->buffer, initial_program);
            }

            HASH_ADD_KEYPTR(hh, node_map, node->buffer, strlen(node->buffer), node);
        }
        else {
            printf("ans: %s\n", saved_cycles[(TOTAL_CYCLES % (cycle_number - 1)) - 1]);
            for (int j = 0; j < s_c_i; j++) {
                free(saved_cycles[j]);
            }
            break;
        }

    }
    delete_all(&node_map);

    fclose(f);
    return 0;
}

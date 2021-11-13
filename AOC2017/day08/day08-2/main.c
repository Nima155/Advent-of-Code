#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "../../uthash-master/src/uthash.h"
#include <limits.h>
#define MAX_LENGTH 30703

#define INSTRUCTION_LENGTHS 3
#define FREE_ALL(array, size) \
       for (int m = 0; m < size; m++) { \
           if (array[m] != NULL)  { \
              free(array[m]);  \
           } \
       }



typedef struct Node Node;

struct Node {
    const char *register_name;
    int *value;
    UT_hash_handle hh;
};

static Node *map = NULL;

bool eval(char *register_name, char *comparison, int second_operand) {
    Node *structure = NULL;

    HASH_FIND_STR(map, register_name, structure);

    if (structure == NULL) {
        structure = malloc(sizeof(Node));

        int *zero = malloc(sizeof(int));
        *zero = 0;

        char *copy = malloc(10);
        strcpy(copy, register_name);

        structure->register_name = copy;
        structure->value = zero;

        HASH_ADD_KEYPTR(hh, map, structure->register_name, strlen(structure->register_name), structure);

    }
    if (strcmp(comparison, "==") == 0)
        return *(structure->value) == second_operand;

    else if (strcmp(comparison, ">=") == 0) {
        return *(structure->value) >= second_operand;
    }

    else if (strcmp(comparison, ">") == 0)
        return *(structure->value)  > second_operand;

    else if (strcmp(comparison, "<") == 0)
        return *(structure->value)  < second_operand;


    else if (strcmp(comparison, "!=") == 0)
        return *(structure->value)  != second_operand;

    else
        return *(structure->value)  <= second_operand;

}

void main_logic(char *instructions[], char *conditions[], int *all_time_max) {
    int move_num = atoi(instructions[2]), cond_number = atoi(conditions[2]);
    bool is_dec = strcmp(instructions[1], "dec") == 0;

    int a = 0;

    if (eval(conditions[0], conditions[1], cond_number)) {

        int len = strlen(instructions[0]);
        Node *structure = NULL;
        HASH_FIND_STR(map, instructions[0], structure);

        if (structure == NULL) {
            structure = malloc(sizeof(Node));

            char *copy = malloc(10);
            strcpy(copy, instructions[0]);

            int *zero = malloc(sizeof(int));
            *zero = 0;

            structure->register_name = copy;
            structure->value = zero;

            HASH_ADD_KEYPTR(hh, map, structure->register_name, strlen(structure->register_name), structure);

        }

        if (structure != NULL) {
            if (is_dec) {
                *(structure->value) -= move_num;
            } else {
                *(structure->value) += move_num;
            }
        }
        *all_time_max =  *all_time_max < *(structure->value)? *(structure->value): *all_time_max;
    }

}
void free_all() {
    Node *current, *tmp;

    HASH_ITER(hh, map, current, tmp) {
        free(current->register_name);
        free(current->value);
        free(current);
    }
}

bool any(char *buffer [], int size) {
    for (int i = 0; i < size; i++) {
        if (buffer[i] == NULL) {
            return true;
        }
    }
    return false;
}

// BEWARE: there may be inadvertent memory leaks in this program
int main()
{
    FILE *f_handle = fopen("input.txt", "r");





    if (f_handle != NULL) {
        int c = 0, i_indx = 0, c_indx = 0, i = 0, all_time_max = INT_MIN;



        char *instructions[] = { malloc(10), malloc(10), malloc(10)  };
        char *conditions[] = { malloc(10), malloc(10), malloc(10) };

        if (any(instructions, INSTRUCTION_LENGTHS) || any(conditions, INSTRUCTION_LENGTHS)) {
                FREE_ALL(instructions, INSTRUCTION_LENGTHS);
                FREE_ALL(conditions, INSTRUCTION_LENGTHS);
        }

        else {
            char temp[10];
            while ((c = fgetc(f_handle)) != EOF) {
                if (c == '\n') {
                    temp[i] = '\0';
                    strcpy(conditions[c_indx], temp);
                    i_indx = 0;
                    c_indx = 0;
                    main_logic(instructions, conditions, &all_time_max);
                    i = 0;
                }

                if (c == ' ' && i_indx < 3) {
                    temp[i] = '\0';
                    strcpy(instructions[i_indx++], temp);
                    i = 0;
                }

                else if (c == ' ' && c_indx < 3) {
                    temp[i] = '\0';

                    if (strcmp(temp, "if") != 0) {
                        strcpy(conditions[c_indx++], temp);
                    }

                    i = 0;
                }
                else if (c != '\n') {
                    temp[i++] = c;
                }


            }

            FREE_ALL(instructions, 3);
            FREE_ALL(conditions, 3);
        }




        free_all();

        printf("answer: %d\n", all_time_max);
        fclose(f_handle);
    }



    return 0;
}

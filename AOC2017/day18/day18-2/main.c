#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "../../uthash-master/src/utlist.h"

#define INPUT_LENGTH 41
#define ALPHABET_LENGTH 26
typedef struct Element Element;

struct Element {
    int sent_value;
    Element *prev; /* needed for a doubly-linked list only */
    Element *next; /* needed for singly- or doubly-linked lists */
};

Element *queue_0 = NULL;
Element *queue_1 = NULL;


typedef struct Instruction Instruction;

struct Instruction {
    char *command;
    char *arg_1;
    char *arg_2;
};
void free_queue(Element **cur_q) {
    Element *elt, *tmp;

    DL_FOREACH_SAFE(*cur_q, elt, tmp) {
          DL_DELETE(*cur_q, elt);
          free(elt);
    }
}
int prog_1_send_count = 0;
void perform_op(long long registers[],
                Instruction **instructions,
                int *instruction_indx, int program_number) {


    Instruction *cur_instruction = instructions[*instruction_indx];
    char *command = instructions[*instruction_indx]->command;


    if (strcmp(command, "snd") == 0) {
        Element *el = malloc(sizeof(Element));

        prog_1_send_count += program_number == 1;
        Element **cur_q = program_number ? &queue_0: &queue_1;
        if (el != NULL) {

            el->sent_value = registers[*(cur_instruction->arg_1) - 'a'];
            DL_APPEND(*cur_q, el);

        }

    }
    else if (strcmp(command, "rcv") == 0) {
        Element **cur_q = program_number ? &queue_1: &queue_0;
        Element *elt;
        int is_empty = 1, value = 0;

        // simple pop operation
        DL_FOREACH(*cur_q, elt) {
              value = elt->sent_value;
              is_empty = 0;
              break;
        }


        if (!is_empty) {

            DL_DELETE(*cur_q, elt);
            free(elt);
            registers[*(cur_instruction->arg_1) - 'a'] = value;
        }
        else {
            return;
        }

    }
    int arg_2 = isdigit(*(cur_instruction->arg_2)) || *(cur_instruction->arg_2) == '-' ?
                atoi(cur_instruction->arg_2): registers[*(cur_instruction->arg_2) - 'a'];

    if (strcmp(command, "mul") == 0) {
        registers[*(cur_instruction->arg_1) - 'a'] *=  arg_2;
    }
    else if (strcmp(command, "add") == 0) {

        registers[*(cur_instruction->arg_1) - 'a'] += arg_2;
    }
    else if (strcmp(command, "jgz") == 0) {

        if (isdigit(*cur_instruction->arg_1)) {
            if ((*(cur_instruction->arg_1) - '0') > 0) {
                *instruction_indx += arg_2;
                return;
            }
        }
        else {

            if (registers[*(cur_instruction->arg_1) - 'a'] > 0) {
                *instruction_indx += arg_2;
                return;
            }
        }

    }
    else if (strcmp(command, "mod") == 0) {
        registers[*(cur_instruction->arg_1) - 'a'] %= arg_2;
    }
    else if (strcmp(command, "set") == 0) {
        registers[*(cur_instruction->arg_1) - 'a'] = arg_2;
    }

    *instruction_indx += 1;
}
int main()
{
    FILE *f = fopen("input.txt", "r");

    Instruction **instructions = malloc(sizeof(Instruction*) * INPUT_LENGTH);


    if (f == NULL) {
        free(instructions);
        return 1;
    }

    long long registers_0[ALPHABET_LENGTH],
              registers_1[ALPHABET_LENGTH];

    int i = 0, cur_instrcution_indx_0 = 0, cur_instrcution_indx_1 = 0;

    for (; i < ALPHABET_LENGTH; i++) {
            registers_0[i] = 0;
            registers_1[i] = ('a' + i) == 'p';
    }

    int c = 0, command_indx = 0, inner_indx = 0, inst_indx = 0;
    char command[3][10];

    while ((c = fgetc(f)) != EOF) {

        if (c == '\n') {

            command[command_indx][inner_indx] = '\0';

            Instruction *cur_instruction = malloc(sizeof(Instruction));

            cur_instruction->command = malloc(4);
            cur_instruction->arg_1 = malloc(2);
            cur_instruction->arg_2 = malloc(8);

            strcpy(cur_instruction->command, command[0]);
            strcpy(cur_instruction->arg_1, command[1]);
            strcpy(cur_instruction->arg_2, command[2]);
            instructions[inst_indx] = cur_instruction;

            command[2][0] = '\0';
            command_indx = 0, inner_indx = 0;
            inst_indx++;

        }
        else if (c == ' ') {
            command[command_indx++][inner_indx] = '\0';
            inner_indx = 0;
        }
        else {
            command[command_indx][inner_indx++] = c;
        }
    }
   int cycle = 0;
   while (1) {
       int bef_1 = cur_instrcution_indx_0, bef_2 = cur_instrcution_indx_1;

       perform_op(registers_0, instructions, &cur_instrcution_indx_0, 0);
       perform_op(registers_1, instructions, &cur_instrcution_indx_1, 1);

       if (bef_1 == cur_instrcution_indx_0 && bef_2 == cur_instrcution_indx_1) {
            break;
       }

   }
   printf("ans: %d\n", prog_1_send_count);
   for (i = 0; i < INPUT_LENGTH; i++) {
        free(instructions[i]->arg_1);
        free(instructions[i]->arg_2);
        free(instructions[i]->command);
        free(instructions[i]);
    }

    free(instructions);
    free_queue(&queue_0);
    free_queue(&queue_1);


    fclose(f);
    return 0;
}

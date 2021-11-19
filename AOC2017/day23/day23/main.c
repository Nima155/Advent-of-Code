#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "../../uthash-master/src/utlist.h"

#define INPUT_LENGTH 32
#define REGISTER_CONTAINER_LENGTH 8


typedef struct Instruction Instruction;

struct Instruction {
    char *command;
    char *arg_1;
    char *arg_2;
};



void perform_op(long long registers[],
                Instruction **instructions,
                int *instruction_indx, int *mul_invocations) {


    Instruction *cur_instruction = instructions[*instruction_indx];
    char *command = instructions[*instruction_indx]->command;


    int arg_2 = isdigit(*(cur_instruction->arg_2)) || *(cur_instruction->arg_2) == '-' ?
                atoi(cur_instruction->arg_2): registers[*(cur_instruction->arg_2) - 'a'];

    if (strcmp(command, "mul") == 0) {
        (*mul_invocations)++;
        registers[*(cur_instruction->arg_1) - 'a'] *=  arg_2;
    }
    else if (strcmp(command, "jnz") == 0) {

        if (isdigit(*cur_instruction->arg_1)) {
            if ((*(cur_instruction->arg_1) - '0') != 0) {
                *instruction_indx += arg_2;
                return;
            }
        }
        else {

            if (registers[*(cur_instruction->arg_1) - 'a'] != 0) {
                *instruction_indx += arg_2;
                return;
            }
        }

    }
    else if (strcmp(command, "sub") == 0) {
        registers[*(cur_instruction->arg_1) - 'a'] -= arg_2;
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

    long long registers[REGISTER_CONTAINER_LENGTH];


    int i = 0, cur_instrcution_indx = 0;



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
   int cycle = 0, mul_invocations = 0;

   while (cur_instrcution_indx >= 0 && cur_instrcution_indx < INPUT_LENGTH) {
       int bef_1 = cur_instrcution_indx;

       perform_op(registers, instructions, &cur_instrcution_indx, &mul_invocations);

   }

   for (i = 0; i < INPUT_LENGTH; i++) {
        free(instructions[i]->arg_1);
        free(instructions[i]->arg_2);
        free(instructions[i]->command);
        free(instructions[i]);
    }

    free(instructions);
    printf("number of mul invocations: %d\n", mul_invocations);


    fclose(f);
    return 0;
}

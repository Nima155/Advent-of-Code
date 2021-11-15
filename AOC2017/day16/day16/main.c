#include <stdio.h>
#include <stdlib.h>
#define PROGRAM_LENGTH 17
// s1, a spin of size 1: eabcd.
// x3/4, swapping the last two programs: eabdc.
// pe/b, swapping programs e and b: baedc.



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


    char initial_program[PROGRAM_LENGTH] = "abcdefghijklmnop";

    char current_instruction[20];

    int c = 0, cur_i_index = 0;

    while (1) {
        c = fgetc(f);
        if (c == ',' || c == EOF) {
            current_instruction[cur_i_index] = '\0';

            switch (current_instruction[0]) {
            case 's':
                spin(current_instruction, initial_program);
                break;
            case 'x':
                xchange(current_instruction, initial_program);
                break;
            case 'p':
                pchange(current_instruction, initial_program);
                break;
            }

            cur_i_index = 0;

            if (c == EOF) {
                break;
            }
        } else {
            current_instruction[cur_i_index++] = c;
        }
    }
    printf("%s\n", initial_program);
    fclose(f);
    return 0;
}

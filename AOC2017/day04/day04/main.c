#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#define MAX_WORDS 20
#define TERMINATOR 'Z'
// djb 2 hash function taken from: http://www.cse.yorku.ca/~oz/hash.html
void free_buffer(char *buffer[]) {
    for (int i = 0; i < MAX_WORDS; i++) {
        if (buffer[i] != NULL) {
            free(buffer[i]);
            buffer[i] = NULL;
        }
    }
}


int check_validity(char *words[]) {
    int ans = 0;

    for (int i = 0; i < MAX_WORDS; i++) {

        for (int j = 0; j < MAX_WORDS; j++) {

            if (i != j && words[i] != NULL && words[j] != NULL && strcmp(words[i], words[j]) == 0) {
                return 0;
            }

            ans = 1;
        }
    }
    return ans;
}

int read_lines_and_sum_valids(FILE *file_handle) {
    char *buffer[MAX_WORDS] = { NULL };
    int indx = 0, c = 0, one_time_indx = 0, sums = 0;

    char *one_time_buffer = malloc(31);

    while ((c = fgetc(file_handle)) != TERMINATOR) {

            if (isalpha(c)) {
                one_time_buffer[one_time_indx++] = c;
            }

            else if (one_time_indx != 0 || c == '\n' || c == EOF) {

                one_time_buffer[one_time_indx] = '\0';
                buffer[indx++] = one_time_buffer;

                if (c != EOF) {
                    one_time_buffer = malloc(31);
                    one_time_indx = 0;
                }


                if (c == '\n' || c == EOF) {
                    //printf("starting to free:");
                    sums += c != EOF ? check_validity(buffer): 0;
                    free_buffer(buffer);
                    indx = 0;

                    if (c == EOF) { break; }
                }



            }
    }

    return sums;
}

int main()
{
    FILE *f = fopen("input.txt", "r");

    if (f == NULL) {
        return 1;
    }

    printf("%d", read_lines_and_sum_valids(f));

    fclose(f);


    return 0;
}

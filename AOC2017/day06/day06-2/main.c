#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <limits.h>
#include <string.h>
#define LENGTH 16
#define VISITED_LENGTH 100000

int buffer[LENGTH] = { 0 };

void read_line_and_fill_buffer(FILE *f) {
    int c = 0, num = 0, i = 0, touched = 0;
    while ((c = fgetc(f)) != EOF) {
        if (isdigit(c)) {
            num *= 10;
            num += (c - '0');
            touched = 1;
        } else if (touched) {
            touched = 0;
            buffer[i++] = num;
            num = 0;
        }
    }
    buffer[i] = num;

}

int max_finder() {
    int max = buffer[0], ans = 0;
    for (int i = 0; i < LENGTH; i++) {
        if (buffer[i] > max) {
            max = buffer[i];
            ans = i;
        }
    }
    return ans;
}

char *int_buffer_to_string() {

    char *str = malloc(100);


    for (int i = 0; i < LENGTH; i++) {
        char temp[10];
        sprintf(temp, "%d,", buffer[i]);
        if (i == 0) {
            strcpy(str, temp);
        } else {
            strcat(str, temp);
        }
    }
    return str;

}

int contains(char *string, char *visited[]) {

    for (int i = 0; i < VISITED_LENGTH; i++) {

        if (visited[i] == NULL) {
            return 0;
        } else if (strcmp(visited[i], string) == 0 ) {
            return 1;
        }
    }

    return 0;
}

void cycle_runner() {
    int visited_indx = 0, steps = 0, first_time = 1, saved_index_of_starting_cycle = -1;

    char *visited[VISITED_LENGTH] = { NULL };

    visited[visited_indx++] = int_buffer_to_string();

    while (1) {
        int max_indx = max_finder();

        int save = buffer[max_indx], cursor = (max_indx + 1) % LENGTH;
        buffer[max_indx] = 0;

        while (save > 0) {

            buffer[cursor]++;
            save--;
            cursor++;
            cursor %= LENGTH;
        }

        steps++;
        char *node = int_buffer_to_string();

        if ((saved_index_of_starting_cycle < 0 && contains(node, visited))
            || (saved_index_of_starting_cycle >= 0 &&
                strcmp(node, visited[saved_index_of_starting_cycle]) == 0)) {

            if (!first_time) {
                free(node);
                break;
            }
            saved_index_of_starting_cycle = visited_indx;
            first_time = 0;
            steps = 0;
        }

        visited[visited_indx++] = node;
    }

    for (int i = 0; i < VISITED_LENGTH; i++) {
        if (visited[i] != NULL) {
            free(visited[i]);
        }
    }
    printf("steps: %d", steps);


}

int main()
{
    FILE *file_handle = fopen("input.txt", "r");

    if (file_handle == NULL) {
        return 1;
    }

    read_line_and_fill_buffer(file_handle);
    cycle_runner();

    fclose(file_handle);


    return 0;
}

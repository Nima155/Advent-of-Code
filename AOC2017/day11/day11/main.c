#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct Point Point;

struct Point {
    int y;
    int x;
    int z;
};

Point interpret_instruction(char buffer[]) {
    Point p = { 0, 0, 0 };

    if (strcmp(buffer, "ne") == 0) {
        p.z = -1;
        p.y = 1;
        return p;
    }
    else if (strcmp(buffer, "se") == 0) {
        p.y = 1;
        p.x = -1;
        return p;
    }
    else if (strcmp(buffer, "n") == 0) {
        p.z = -1;
        p.x = 1;
        return p;
    }
    else if (strcmp(buffer, "s") == 0) {
        p.x = -1;
        p.z = 1;
        return p;
    }
    else if (strcmp(buffer, "sw") == 0) {
        p.z = 1;
        p.y = -1;
        return p;
    }
    else if (strcmp(buffer, "nw") == 0) {
        p.y = -1;
        p.x = 1;
        return p;
    }

}


void read_and_interpret(FILE *f) {
    char buffer[5];
    int buffer_indx = 0;
    int y = 0, x = 0, z = 0, c = 0, mx = 0;
    while ((c = fgetc(f))) {

        if (c != ',' && c != EOF && c != '\n') {
            buffer[buffer_indx++] = c;
        }
        else {
            buffer[buffer_indx] = '\0';
            buffer_indx = 0;
            Point p = interpret_instruction(buffer);

            y += p.y;
            x += p.x;
            z += p.z;

            if (c == EOF || c == '\n')
                break;
        }
        mx = mx > y+z ? mx: y + z;
    }
    printf("part1: %d part2: %d\n", y + z, mx);

}


int main()
{
    FILE *file_handle = fopen("input.txt", "r");

    if (file_handle == NULL) {
        return 1;
    }

    read_and_interpret(file_handle);

    fclose(file_handle);

    return 0;
}

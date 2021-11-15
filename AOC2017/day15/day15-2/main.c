#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#define GEN_A_FACTOR 16807
#define GEN_B_FACTOR 48271
#define TOTAL_CYCLES 5000000
#define MODULO 2147483647

char *to_bin(long long a) {
    char *bin_rep = malloc(17);
    if (bin_rep == NULL) {
        return NULL;
    }

    int indx = 0;
    do {
       bin_rep[indx++] = (a % 2) + '0';
       a /= 2;
    } while (a > 0 && indx < 16);
    bin_rep[indx] = '\0';

    return bin_rep;
}


bool is_matching(long long a, long long b) {
    char *bin_1 = to_bin(a);
    char *bin_2 = to_bin(b);

    bool equal = strcmp(bin_1, bin_2) == 0;
    free(bin_1);
    free(bin_2);
    return equal;
}

long long value_generator(long long num, long long multiple, long long factor) {

    do {
        num = (num * factor) % MODULO;
    } while(num % multiple != 0);
    return num;
}

int main()
{
    int cycles = 0, matching_pairs = 0;

    long long a = 634, b = 301;



    while (cycles < TOTAL_CYCLES) {
        matching_pairs += is_matching(a, b);
        a = value_generator(a, 4, GEN_A_FACTOR);
        b = value_generator(b, 8, GEN_B_FACTOR);
        cycles++;
    }


    printf("matching pairs: %d\n", matching_pairs);
}

#include <stdio.h>
#include <stdlib.h>


int main (){
    FILE *f = fopen("input.txt", "r");

    if (f == NULL) {
        return 1;
    }

    int buff[3000] = { 0 };
    char temp = ' ';
    int i = 0;


    while ((temp = fgetc(f)) != '\n') {

        buff[i] = temp - '0';
        i++;
    }

    fclose(f);
    printf("%d", main_logic(buff, i / 2, i));


}

int main_logic(int buffer[], int middle, int len) {

    int i = 0, ans = 0, do_break = 0;

    while (1) {

        int next = buffer[(i + middle) % len];

        if (buffer[i] == 0) {
            break;
        }


        ans += next == buffer[i] ? buffer[i]: 0;
        i++;
    }

    return ans;

}

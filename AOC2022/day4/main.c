//
// Created by Nima on 12/16/2023.
//
int doubleComp(int times[]) {
    if ((times[0] >= times[2] && times[1] <= times[3]) || times[2] >= times[0] && times[3] <= times[1]) return 1;
    return 0;
}
int overLap(int times[]) {
    if ((times[0] >= times[2] && times[3] >= times[0]) || times[2] >= times[0] && times[1] >= times[2]) {
        return 1;
    }
    return 0;
}

#include <stdio.h>
int main() {
    FILE *file = fopen("input.txt", "r");
    if (file) {
        int times[] = {0, 0, 0, 0};
        int ans = 0, ansP2 = 0;
        while (fscanf(file, "%d-%d,%d-%d", &times[0], &times[1], &times[2], &times[3]) != EOF) {
            ans += doubleComp(times);
            ansP2 += overLap(times);
        };
        printf("%d %d\n", ans, ansP2);
        fclose(file);
    }
    return 0;
}
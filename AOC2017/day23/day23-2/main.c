#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>
#include <math.h>

bool is_prime(int n) {
    int lim = sqrt(n);

    for (int i = 2; i < lim; i++) {
        if (n % i == 0) return false;
    }
    return true;
}



int main()
{


    long long b = 108400, c = 125400, a = 1, d = 2, e = 2, f = 1, g = 0, h = 0;

    int ans = 0;
    for (int i = b; i <= c; i += 17) {

        ans += !is_prime(i);
    }
    printf("%d\n", ans);
} // number of numbers between 108400 and 125400 with steps of size 17 that are not prime
//
//    do {
//        f = 1;
//        d = 2;
//        do {
//            e = 2;
//            do {
//                g = d;
//                g *= e;
//                g -= b;
//                if (g == 0) {
//                    f = 0;
//                    printf("%d %d %d %d %d %d %d %d\n", a, b, c, d, e, f, g, h);
//                }
//                e -= -1;
//                g = e;
//                g -= b;
//            } while (g != 0);
//            d -= -1;
//            g = d;
//            g -= b;
//         } while (g != 0);
//         if (f == 0) {
//            printf("b: %d\n", b);
//            h -= -1;
//         }
//         g = b;
//         g -= c;
//         if (g != 0) {
//            b -= -17;
//         }
//
//    } while (g != 0);

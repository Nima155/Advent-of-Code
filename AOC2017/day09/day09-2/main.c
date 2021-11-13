#include <stdio.h>
#include <stdlib.h>

#define NORMAL_MODE 0
#define GARBAGE_MODE 1
#define BANG_MODE 2

int main()
{
    FILE *f_handle = fopen("input.txt", "r");

    if (f_handle == NULL) {
        return 1;
    }

    int current_char = 0, cur_mode = NORMAL_MODE, prev_mode = NORMAL_MODE, nesting_level = 0;


    int ans = 0;
    while ((current_char = fgetc(f_handle)) != EOF) {

          if (cur_mode == GARBAGE_MODE && current_char == '!') {
                prev_mode = cur_mode;
                cur_mode = BANG_MODE;
          }
          else if (cur_mode == GARBAGE_MODE && current_char != '>') {
                ans++;
          }
          else if (cur_mode == NORMAL_MODE && current_char == '<') {
                cur_mode = GARBAGE_MODE;
          }
          else if (cur_mode == GARBAGE_MODE && current_char == '>') {
                cur_mode = NORMAL_MODE;
          }
          else if (cur_mode == NORMAL_MODE && current_char == '{') {
                nesting_level++;
          }
          else if (cur_mode == NORMAL_MODE && current_char == '}') {
                nesting_level--;
          }
          else if (cur_mode == BANG_MODE) {
                cur_mode = prev_mode;
          }

    }
    printf("%d\n", ans);
    fclose(f_handle);

    return 0;
}

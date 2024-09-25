#include "rhiz_tag.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    (void)argc;
    (void)argv;
    char buf[4];
    char input[64];
    int64_t unix_time;

    while (fgets(input, sizeof(input), stdin) != NULL) {
        if (sscanf(input, "%lld", &unix_time) != 1) {
            printf("Invalid input\n");
            fflush(stdout);
            return 1;
        }

        if (to_datetag(buf, sizeof(buf), unix_time)) {
            printf("Could not convert\n");
            fflush(stdout);
            return 1;
        }

        printf("%s\n", buf);
          fflush(stdout);
    }

    return 0;
}

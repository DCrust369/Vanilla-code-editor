#include <stdio.h>
#include <stdbool.h>

struct everything_is_file {
    bool your_archives;
    int file_descriptor;
};

int main(void) {
    int archive_val = 0;
    int file_val = 0;
    
    int *archive = &archive_val;
    int *file = &file_val;

    struct everything_is_file item;
    item.your_archives = true;

    return 0;
}

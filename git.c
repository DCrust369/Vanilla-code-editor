#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
int32_t* terminal_instructions(void) {
    int32_t *ptr = malloc(sizeof(int32_t));
    *ptr = 100;
    return ptr;
}

int main(void)
{
    bool add = true;
    bool push = true;
    bool init = true;

    struct gitea {
        int32_t *golang;
        int32_t *other_data;
    };

    struct gitea my_gitea;
    my_gitea.golang = terminal_instructions();
    my_gitea.other_data = malloc(sizeof(int32_t));

    free(my_gitea.golang);
    free(my_gitea.other_data);

    return 0;
}

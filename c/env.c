#include <stdio.h>

extern char **environ;

int main(void){
    for (char **env = environ; *env; env++){
        printf("%s\n", *env);
    }
}

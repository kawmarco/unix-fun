/*
 * Simple ls implementation, mostly from [APUE]
 */
#include <stdio.h>
#include <stdlib.h>
#include <dirent.h>

#include "unixfun.h"

int main(int argc, char* argv[]){
    DIR *dp;
    struct dirent *dirp;

    if (argc != 2) {
        fprintf(stderr, "usage: %s <dir>\n", argv[0]);
        exit(1);
    }

    if ((dp = opendir(argv[1])) == NULL){
        fprintf(stderr, "could not open %s\n", argv[1]);
        exit(1);
    }

    while ((dirp = readdir(dp)) != NULL) {
        // as of Linux 4.13, d_name is limited to 
        // NAME_MAX = 255 chars
        // https://github.com/tinganho/linux-kernel/blob/f1349b033d5faee3682eea244a7cb4dbbb7982d0/include/linux/limits.h#L11
        debugger();
        printf("%s\n", dirp->d_name);
    }

    closedir(dp);
    exit(0);
}




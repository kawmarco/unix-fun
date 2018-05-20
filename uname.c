#include <sys/utsname.h>
#include <stdio.h>
#include "unixfun.h"

int main(void){
    struct utsname un;

    if(uname(&un) == 0){
        printf("sysname: %s\n", un.sysname);
        printf("nodename: %s\n", un.nodename);
        printf("release: %s\n", un.release);
        printf("version: %s\n", un.version);
    } else {
        perror("Could not run uname()");
    }
}

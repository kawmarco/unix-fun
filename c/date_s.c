/*
 * Just a program that emulates `date +%s`
 */

#include <time.h>
#include <stdio.h>

int main(void){
    printf("%ld\n", time(0));
}

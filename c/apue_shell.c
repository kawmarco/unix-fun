/*
 * Chapter 1 example shell from APUE
 */
#include <sys/wait.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

// Ideally should be equal to MAX_ARG_STRLEN on Linux
// https://www.in-ulm.de/~mascheck/various/argmax/
#define MAXLINE 1024

int main(void){
    char buf[MAXLINE];
    pid_t pid;
    int status;

    printf("$ ");

    while (fgets(buf, MAXLINE, stdin) != NULL) {
        if (buf[strlen(buf) - 1] == '\n') {
            buf[strlen(buf) - 1] = 0;
        }

        if ((pid = fork()) < 0){
            fprintf(stderr, "fork error\n");
            exit(1);
        } else if (pid == 0) {
            // child
            // XXX why execlp?
            execlp(buf, buf, (char*) 0);
            fprintf(stderr, "couldn't execute %s\n", buf);
            exit(127);
        }

        // parent
        if ((pid = waitpid(pid, &status, 0)) < 0) {
            fprintf(stderr, "waitpid error");
        }

        printf("$ ");
    }
    exit(0);
}


#include <stdlib.h>
#include <stdio.h>

void say_hello()
{
    printf("say_hello: address of function malloc() is :%p\n", malloc); 

    char *buf = malloc(1024);
    if(NULL != buf)
    {
        snprintf(buf, 1024, "%s", "hello\n");
        printf("%s", buf);
    }
}

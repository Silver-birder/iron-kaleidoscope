#include <stdio.h>

int sum(int l, int r)
{
    return l + r;
}

int main(int argc, char **argv)
{
    int l = 1;
    int r = 2;
    int result = sum(l, r);
    printf("%d", result);
    return 0;
}
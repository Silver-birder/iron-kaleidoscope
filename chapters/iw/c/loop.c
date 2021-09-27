#include <stdio.h>

void loop(int n)
{
     int count;
     for (count = 1; count <= n; count = count + 1)
          printf("hello\n");
}
int main(void)
{
     printf("Please input the number.\n");

     int i;
     scanf("%d", &i);

     loop(i);

     return 0;
}
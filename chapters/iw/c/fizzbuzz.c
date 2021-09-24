#include <stdio.h>

void fizzbuzz(int n)
{
     if (n % 3 == 0 && n % 5 == 0)
     {
          printf("Fizz,Buzz\n");
     }
     else if (n % 3 == 0)
     {
          printf("Fizz\n");
     }
     else if (n % 5 == 0)
     {
          printf("Buzz\n");
     }
}

int main(void)
{
     printf("Please input the number.\n");

     int i;
     scanf("%d", &i);

     fizzbuzz(i);

     return 0;
}
const fizzBuzz = (n) => {
    switch (true) {
        case n % 15 == 0:
            console.log('fizzbuzz');
            break;
        case n % 3 == 0:
            console.log('fizz');
            break;
        case n % 5 == 0:
            console.log('buzz');
            break;
    }
}

fizzBuzz(30);

const addon = require('./index.node');

start = Date.now();
for (let i = 0; i < 1000; i++) {
    addon.factorial(20);
}
end = Date.now();
console.log('Rust - Factorial - Time elapsed = ', end - start);

start = Date.now();
for (let i = 0; i < 1000; i++) {
    result = 1
    for(let j = 0; j < 20; j++) {
        result *= j;
    }
}
end = Date.now();
console.log('Native - Factorial - Time elapsed = ', end - start);


start = Date.now();
data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]
for (let i = 0; i < 100; i++) {
    addon.rayon(data);
}
end = Date.now();
console.log('Rust - SumOfSquares - Time elapsed = ', end - start);

start = Date.now();
for (let i = 0; i < 100; i++) {
    sum = 0
    for(const el of data) {
        sum += el * el;
    }
}
end = Date.now();
console.log('Native - SumOfSquares - Time elapsed = ', end - start);
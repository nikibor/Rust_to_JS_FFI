
const addon = require('./index.node');
console.log(addon.factorial(5));
console.log(addon.hello());

start = Date.now();
data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
for (let i = 0; i < 100; i++) {
    addon.rayon(data);
}
end = Date.now();
console.log('Rust - Time elapsed = ', end - start);

start = Date.now();
for (let i = 0; i < 100; i++) {
    sum = 0
    for(let j = 0; j < 10; j++) {
        sum += data[j] * data[j]
    }
}
end = Date.now();
console.log('Native - Time elapsed = ', end - start);
//cx.export_function("factorial", factorial)?;

const addon = require('./index.node');
console.log(addon.factorial(5));

import python_bindings

print(python_bindings.sum_as_string(2, 2))

print(python_bindings.factorial([1, 2, 3, 4, 5, 6, 7, 8, 9]))

print(python_bindings.read_json_configs(
    "/Users/nikitaborgolov/Documents/Projects/Rust_to_JS_FFI/test_configs"))

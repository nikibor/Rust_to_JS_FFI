import datetime
import os
import python_bindings
import json
import jsonschema


def validate_json_file(file_path):
    schema = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "apiVersion": {
                "type": "string"
            },
            "kind": {
                "type": "string"
            },
            "clusters": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "proxy-url": {
                            "type": "string",
                            "format": "uri"
                        },
                        "server": {
                            "type": "string",
                            "format": "uri"
                        }
                    },
                    "required": ["proxy-url", "server"]
                }
            },
            "name": {
                "type": "string"
            },
            "users": {
                "type": "array",
                "items": {
                    "type": "string"
                }
            }
        },
        "required": ["apiVersion", "kind", "clusters", "name", "users"]
    }
    try:
        for filename in os.listdir(file_path):
            # Load the JSON data from the file
            file_path = file_path+'/'+filename
            with open(file_path, 'r') as file:
                json_data = json.load(file)

            # Validate the JSON data against the schema
            jsonschema.validate(instance=json_data, schema=schema)

        return True, "JSON data is valid according to the schema."
    except jsonschema.exceptions.ValidationError as e:
        return False, f"Validation error: {e}"
    except json.JSONDecodeError as e:
        return False, f"JSON decoding error: {e}"
    except FileNotFoundError:
        return False, "File not found."
    except Exception as e:
        return False, f"An error occurred: {str(e)}"


def sum_of_squares(n):
    return sum(x**2 for x in n)


# print(python_bindings.sum_as_string(2, 2))

data_len = [10, 50, 100, 200, 250]

# for d_l in data_len:
#     data = [*range(d_l)]
#     # RUST
#     time_start = datetime.datetime.now()
#     for _ in range(100_000):
#         python_bindings.factorial(data)

#     time_end = datetime.datetime.now()
#     time_delta = (time_end - time_start).microseconds
#     print("Rust arr of len " + str(d_l) + " runs in " +
#           str(time_delta) + " microseconds after 100_000 iterations")

#     # PYTHON
#     time_start = datetime.datetime.now()
#     for _ in range(100_000):
#         sum_of_squares(data)

#     time_end = datetime.datetime.now()
#     time_delta = (time_end - time_start).microseconds
#     print("Python arr of len " + str(d_l) + " runs in " +
#           str(time_delta) + " microseconds after 100_000 iterations")


# RUST
# time_start = datetime.datetime.now()
# for _ in range(100_000):
#     python_bindings.read_json_configs(
#         "/Users/nikitaborgolov/Documents/Projects/Rust_to_JS_FFI/test_configs")

# time_end = datetime.datetime.now()
# time_delta = (time_end - time_start).microseconds
# print("Rust reading 100_000 json files runs in " +
#       str(time_delta) + " microseconds after 100_000 iterations")

# PYTHON
time_start = datetime.datetime.now()
for _ in range(100_000):
    validate_json_file(
        "/Users/nikitaborgolov/Documents/Projects/Rust_to_JS_FFI/test_configs")
time_end = datetime.datetime.now()
time_delta = (time_end - time_start).microseconds
print("Python reading 100_000 json files runs in " +str(time_delta) + " microseconds after 100_000 iterations")

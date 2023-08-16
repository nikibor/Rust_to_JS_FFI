
"""
config_example = {
    'apiVersion': 'v1',
    'kind': 'Config',
    'clusters': [
        {
            'proxy-url': 'http://proxy.example.org:3128',
            'server': 'https://k8s.example.org/k8s/clusters/c-xxyyzz'
        },
    ],
    'name': 'development',
    'users': ['developer'],
}
"""


import json
import os
import random
import string
import sys

DIR_PATH = 'test_configs'


def get_random_string(length):
    # With combination of lower and upper case
    result_str = ''.join(random.choice(string.ascii_letters)
                         for i in range(length))
    # print random string
    return result_str


if __name__ == '__main__':
    print("### CONFIG GENERETOR ###")
    args = sys.argv
    rows = 5

    if len(args) > 1:
        if args[1]:
            rows = int(args[1])
    print("# Num of configs:", rows)

    if os.path.exists(DIR_PATH):
        os.rmdir(DIR_PATH)
    
    os.mkdir(DIR_PATH)
    print("# All configs are stored at:", DIR_PATH)

    for i in range(rows):
        api_version = 'v{0}'.format(random.randint(1, 10))
        clusters = [{
            'proxy_url': 'http://proxy.{0}.org:{1}'.format(get_random_string(5), random.randint(450, 1000)),
            'server': 'https://k8s.{0}.org/k8s/{1}'.format(get_random_string(5), get_random_string(random.randint(3, 10)))
        } for _ in range(random.randint(5, 20))]
        users = [get_random_string(random.randint(3, 8))
                 for _ in range(random.randint(1, 10))]

        config = {
            'apiVersion': api_version,
            'kind': 'Config',
            'clusters': clusters,
            'name': get_random_string(random.randint(3, 8)),
            'users': users
        }

        file_name = "config_{0}.json".format(i)
        with open('{0}/{1}'.format(DIR_PATH, file_name), 'w') as conf_file:
            conf_file.write(json.dumps(config, indent=4))
        print("# File {0}/{1} - {2} is ready".format(i, rows, file_name))

    print("### ALL FILES ARE PROCESED SUCCESSFULLY ###")

#!/usr/bin/env python3

import argparse
import os
from time import time


def int2base(num, base, numerals="0123456789abcdefghijklmnopqrstuvwxyz"):
    if num == 0:
        return numerals[0]
    else:
        _ = int2base(num // base, base, numerals).lstrip(numerals[0])
        return _ + numerals[num % base]


def generate_version():
    return int2base(int(time()), 36)


def main():
    parser = argparse.ArgumentParser(description='Version number generator.')

    parser.add_argument('--silent', '-s',
                        action='store_true',
                        help='do not print version')

    parser.add_argument('file',
                        metavar='FILE',
                        nargs='?',
                        help='file for save version')

    args = parser.parse_args()
    version = generate_version()

    file_name = (args.file or
                 os.environ.get('VERSIONS_FILE') or
                 os.environ.get('PLUGIN_FILE'))  # for drone.io plugin

    if file_name:
        dir_name = os.path.dirname(file_name)
        if dir_name and not os.path.isdir(dir_name):
            os.makedirs(dir_name, exist_ok=True)

        with open(file_name, 'w') as f:
            f.write(version)

    if not args.silent:
        print(version)


if __name__ == '__main__':
    main()

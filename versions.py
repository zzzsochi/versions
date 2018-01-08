#!/usr/bin/env python3

import argparse
import os
import sys
from time import time

VERSION = '2.0'

ENV_PERFIXES = ['VERSIONS_', 'PLUGIN_']


def int2base(num, base, numerals="0123456789abcdefghijklmnopqrstuvwxyz"):
    if num == 0:
        return numerals[0]
    else:
        _ = int2base(num // base, base, numerals).lstrip(numerals[0])
        return _ + numerals[num % base]


def generate_version():
    return int2base(int(time()), 36)


def _get_fix(arg, env_variable):
    if arg is not None:
        fix = arg
    else:
        for var in [epx + env_variable for epx in ENV_PERFIXES]:
            fix = os.environ.get(var, '')
            if fix:
                break

    if fix and len(fix) > 1 and fix[0] == '@':
        with open(fix[1:]) as f:
            fix = f.read().strip()

    return fix


def main():
    parser = argparse.ArgumentParser(description='Version number generator.')

    parser.add_argument('--prefix',
                        metavar='STRING',
                        default=None,
                        help='prefix for version')

    parser.add_argument('--suffix',
                        metavar='STRING',
                        default=None,
                        help='suffix for version')

    parser.add_argument('--silent', '-s',
                        action='store_true',
                        help='do not print version')

    parser.add_argument('--version',
                        action='store_true',
                        help='print version :-)')

    parser.add_argument('file',
                        metavar='FILE',
                        nargs='?',
                        help='file for save version')

    args = parser.parse_args()

    if args.version:
        print("Versions version {} :-)".format(VERSION))
        sys.exit(0)

    file_name = (args.file or
                 os.environ.get('VERSIONS_FILE') or
                 os.environ.get('PLUGIN_FILE'))  # for drone.io plugin

    prefix = _get_fix(args.prefix, 'PREFIX')
    suffix = _get_fix(args.suffix, 'SUFFIX')

    version = prefix + generate_version() + suffix

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

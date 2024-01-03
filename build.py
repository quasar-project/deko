#!/usr/bin/python3

import os

if __name__ == "__main__":
    os.system("cargo build --release")
    os.system("cd target")
    os.mkdir("dll")
    
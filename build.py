#!/usr/bin/python3

import os
from sys import platform

if __name__ == "__main__":
    os.system("cargo build --release")
    os.chdir("target")
    os.system("mkdir dll")
    if platform == "linux":
        os.system("cp ./release/libdeko.so ./dll")
    if platform == "win32":
        os.system("copy .\\release\\deko.dll .\\dll\\deko.dll")

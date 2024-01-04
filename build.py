#!/usr/bin/python3

import os
from sys import platform

if __name__ == "__main__":
    os.system("cargo build --release")
    os.chdir("..")
    os.chdir("..")
    os.chdir("..")
    print(os.getcwd())
    os.chdir("target")
    os.system("mkdir dll")
    if platform == "linux":
        os.system("cp ./release/libdeko.so ./dll")
    if platform == "win32":
        os.system("copy .\\release\\deko.dll .\\dll\\deko.dll")
    os.chdir("../")
    os.chdir("bindings/c++")
    os.system("mkdir plugins")
    os.chdir("plugins")
    if platform == "linux":
        os.system("mkdir Linux")
        os.chdir("Linux")
        os.system("mkdir x64")
        os.system("cp ../../../../target/dll/libdeko.so ./x64/libdeko.so")
    if platform == "win32":
        os.system("mkdir Windows")
        os.chdir("Windows")
        os.system("mkdir x64")
        os.system("copy ..\\..\\..\\..\\target\\dll\\deko.dll .\\x64\\deko.dll")

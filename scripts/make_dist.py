#!/usr/bin/python3.9
import shutil, os, sys, pathlib

# check if romfs exists
if not os.path.exists("../../romfs.zip"):
  print("ERROR: no romfs zip!")
  exit("romfs.zip was missing!")

# if distribution folder exists, delete it
if "build" in os.listdir('..'):
  shutil.rmtree('../build')
os.makedirs('../build/')
shutil.unpack_archive("../../romfs.zip", "../build/")
shutil.copytree("./target/aarch64-skyline-switch/release/libthe_wubor_patch.nro", "../build/ultimate/mods/The WuBor Patch/plugin.nro")

# zip each folder in the staging dir
shutil.make_archive("The_WuBor_Patch", 'zip', '../build/ultimate')
shutil.move("The_WuBor_Patch.zip", "../The_WuBor_Patch.zip")
#!/usr/bin/env python3

import os



def get_Cargo_path():
    path = os.getcwd()
    while path != "/":
        if os.path.exists(f"{path}/Cargo.toml"):
            return path
        path = os.path.dirname(path)
    return None


def get_first_uncreated_day():
    i = 1
    while True:
        if not os.path.exists(src_day + str(i)):
            return i
        i += 1

cargo_path = get_Cargo_path();
src = f"{cargo_path}/src/"
src_day = f"{src}day"
script_resources = f"{cargo_path}/scripts/resources/"
new_day = get_first_uncreated_day()


print(f"Creating day {new_day}...")
os.makedirs(f"{src_day}{new_day}")
print(f"Created day @ {src_day}{new_day}")
for i in range(1,3):
    part = f"part{i}"
    filename = f"{script_resources}partY.rs"
    file_contents = ""
    with open(filename) as f:
        file_contents = f.read()
    file_contents = file_contents.replace("Y", part)
    file_contents = file_contents.replace("X", str(new_day))
    filename = f"{src_day}{new_day}/{part}.rs"
    with open(filename, "w") as f:
        f.write(file_contents)
# non repeating operations
filename = f"{script_resources}mod.rs"
with open(filename) as f:
    file_contents = f.read()
filename = f"{src_day}{new_day}/mod.rs"
with open(filename, "w") as f:
    f.write(file_contents)
with open(f"{src_day}{new_day}/input.txt", "w") as f:
    pass
with open(f"{src}/main.rs", "r+") as f:
    contents = f.read()
    f.seek(0)
    f.write(f"mod day{new_day};\n"+contents)
print("Day Created, have fun !!")


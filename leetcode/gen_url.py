import os
import fileinput

for (root, _, files) in os.walk("src", topdown=True):
    print(root)
    # print (dirs)
    # print (files)
    print("--------------------------------")
    for file in files:
        path = root + "/" + file
        print(file)
        file = "-".join(file.strip(".rs").split("_")[2:])

        table = f"// https://leetcode.cn/problems/{file}/\n"
        print(table)

        for line in fileinput.input(path, inplace=1):
            line = table + line
            print(line.strip("\n"))
            table = ""

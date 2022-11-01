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
        if file.endswith("2"):
            file=file.strip("2")+"ii"
        if file.endswith("3"):
            file=file.strip("3")+"iii"
        if file.endswith("4"):
            file=file.strip("4")+"iv"

        table = f"// https://leetcode.cn/problems/{file}/\n"
        print(table)

        for line in fileinput.input(path, inplace=1):
            if 'https://leetcode' in line:
                continue
            line = table + line
            print(line.strip("\n"))
            table = ""

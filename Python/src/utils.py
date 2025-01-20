import os

def read_file (relative_path):
        absolute_path = os.path.abspath(relative_path)
        print(absolute_path)

        f = open(absolute_path, "r")
        return f.read()
import os


def scan_repo(root="."):
    files = []

    for path, _, filenames in os.walk(root):
        for f in filenames:
            files.append(os.path.join(path, f))

    return files

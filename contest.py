#!/usr/bin/env python3

# example
# ./contest.py abc206 b c

import sys
import os
import shutil

if len(sys.argv) <= 2:
    print(
        "usage  : ./contest.py [contest_name] [task_name_list]", file=sys.stderr)
    print("example: ./contest.py abc264 b c", file=sys.stderr)
    sys.exit(1)


contest_name: str = sys.argv[1]
problems: list = sys.argv[2:]


# ソースコードの準備(ディレクトリの生成)
dir_path: str = f"src/contest/{contest_name}/"
if not os.path.exists(dir_path):
    os.mkdir(dir_path)

# ソースコードの準備(ファイルのコピー)
templete_file_path: str = "src/contest/template.rs"
for problem in problems:
    dst_file_path = f"src/contest/{contest_name}/main{problem}.rs"
    if not os.path.exists(dst_file_path):
        shutil.copy(templete_file_path, dst_file_path)

def insertList(index: int, base_list: list, insert_list: list):
    before = base_list[:index+1]
    after = base_list[index+1:]
    result = before + insert_list + after
    return result

# cargo.tomlコードの出力
with open('Cargo.toml', 'r+', encoding='utf-8') as file:
    lines = file.readlines()
    insert_index: int = lines.index('path = "src/main.rs"\n') 
    insert_lines: list = []

    for problem in problems:
        name = f"""name = "{contest_name}_{problem}"\n"""
        path = f"""path = "src/contest/{contest_name}/main{problem}.rs"\n""" 
        extend_list = ['\n', '[[bin]]\n', name, path]
        if name not in lines:
            insert_lines.extend(extend_list)
            print(
                f"""
----- Add bin content -----
[[bin]]
name = "{contest_name}_{problem}"
path = "src/contest/{contest_name}/main{problem}.rs"
                """
            )
    
    result_lines = insertList(insert_index, lines, insert_lines)
    file.truncate(0)
    file.seek(0, os.SEEK_SET)
    file.writelines(result_lines)
    
file.close()
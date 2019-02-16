#!/usr/bin/python3

import subprocess
import sys


def build():
    subprocess.call("cargo build", shell=True)


def test(args, expected):
    command = ["./target/debug/llvm_test"] + args
    p = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    out, err = p.communicate()

    ir = err.decode("utf-8")
    with open("./compiled.ll", "w") as f:
        f.write(ir)

    subprocess.call("llvm-as compiled.ll", shell=True)
    p = subprocess.Popen("lli compiled.bc".split(), stdout=subprocess.PIPE)
    streamdata = p.communicate()[0]

    print(f"{args} -> {p.returncode}")
    assert p.returncode == expected


def clean():
    subprocess.call("rm compiled*", shell=True)


def run():
    build()
    test(["10", "20"], 30)
    # test(["111", "222"], 333)


def main(command):
    if command == "--clean":
        clean()
    else:
        run()


if __name__ == "__main__":
    args = sys.argv
    command = args[1] if len(args) > 1 else None
    main(command)

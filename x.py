#!/usr/bin/python3

import subprocess
import sys

def run():
    subprocess.call('cargo run 2>temp', shell=True)
    with open('./temp', 'r') as f:
        res = f.read()
    subprocess.call('rm temp', shell=True)

    # remove messages by cargo
    ir = '\n'.join(res.split('\n')[4:-1])

    with open('./compiled.ll', 'w') as f:
        f.write(ir)

    subprocess.call('llvm-as compiled.ll', shell=True)
    p = subprocess.Popen('lli compiled.bc'.split(), stdout=subprocess.PIPE)
    streamdata = p.communicate()[0]
    print(p.returncode)


def clean():
    subprocess.call('rm compiled*', shell=True)
    
def main(command):
    if command == '--clean':
        clean()
    else:
        print('run')
        run()
    

if __name__ == '__main__':
    args = sys.argv
    command = args[1] if len(args) > 1 else None
    main(command)

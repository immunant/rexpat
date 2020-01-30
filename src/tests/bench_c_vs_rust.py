#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import re
import sys
import argparse
import subprocess as sp


my_commands = \
"""
cargo run --release --bin benchmark -- -n upstream/testdata/largefiles/ns_att_test.xml 16384 16
16 loops, with buffer size 16384. Average time per loop: 0.627948

cargo bench --bench benchmark -- -n upstream/testdata/largefiles/ns_att_test.xml 16384 16
16 loops, with buffer size 16384. Average time per loop: 0.637239
"""

class BenchContext(object):

    bench_output_re = re.compile(
        r"(\d+) loops, with buffer size (\d+)\. Average time per loop: ([\d.]+)\\n"
    )
    n = "64"        # number of benchmark iterations
    buff = "16384"  # buffer size

    def __init__(self):
        self.args =  parse_args()

        # TODO: check that `benchmark` binary exists
        script_dir: str = os.path.dirname(os.path.realpath(__file__))
        self.root_dir = os.path.abspath(
            os.path.join(
                script_dir, 
                "../.."
            )
        )
        assert os.path.isdir(self.root_dir)
        self.benchmark_c = os.path.join(self.root_dir, "upstream/expat/tests/benchmark/benchmark")
        assert os.path.isfile(self.benchmark_c),  f"C benchmark file missing: {self.benchmark_c}"

        # NOTE: don't require this file to exist, cargo run can create it
        self.benchmark_rs = os.path.join(self.root_dir, "target/release/benchmark")
        
        # figure out where the test inputs are
        self.input_dir = os.path.join(self.root_dir, "upstream/testdata/largefiles")
        files = [
            "nes96.xml",
            "ns_att_test.xml",
            "recset.xml",
            "wordnet_glossary-20010201.rdf",
        ]
        self.input_files = [os.path.join(self.input_dir, f) for f in files]
        for inf in self.input_files:
            assert os.path.isfile(inf), f"not found: {inf}"


def parse_args():
    """
    define and parse command line arguments.
    """
    desc = 'Compare the performance of the C version of libexpat to Rust port.'
    parser = argparse.ArgumentParser(description=desc)
    # parser.add_argument('-c', '--clean-all', default=False,
    #                     action='store_true', dest='clean_all',
    #                     help='clean everything before building')
    return parser.parse_args()
    
def have_cargo():
    args = ["cargo", "--version"]
    try:
        cp: sp.CompletedProcess = sp.run(args, capture_output=True)
        assert re.search(r"cargo.*\([:a-z0-9:]+\s.*\)\\n", str(cp.stdout))
        assert len(cp.stderr) == 0
    except AssertionError:
        print("error, cargo not in path.", file=sys.stderr)
        exit(1)

def parse_benchmark_output(ctx: BenchContext, output: str) -> float:
    match = ctx.bench_output_re.search(output)
    assert match
    n = match.group(1)
    buff = match.group(2)
    
    assert ctx.n == n
    assert ctx.buff == buff
    
    return float(match.group(3))


def benchmark_c_code(ctx: BenchContext, inputf:str) -> float:

    args = [ctx.benchmark_c, "-n", inputf, ctx.buff, ctx.n]
    cp: sp.CompletedProcess = sp.run(args, capture_output=True)
    assert not len(cp.stderr)
    
    return parse_benchmark_output(ctx, str(cp.stdout))


def benchmark_rs_code(ctx: BenchContext, inputf:str) -> float:
    if not os.path.isfile(ctx.benchmark_rs):
        args = ["cargo", "build", "--release", "--bin", "benchmark"]
        cp: sp.CompletedProcess = sp.run(args, capture_output=True, cwd=ctx.root_dir)        
        assert cp.returncode == 0
        assert len(cp.stdout) == 0
    
    args = [ ctx.benchmark_rs, "-n", inputf, ctx.buff, ctx.n]

    cp: sp.CompletedProcess = sp.run(args, capture_output=True, cwd=ctx.root_dir)
        
    return parse_benchmark_output(ctx, str(cp.stdout))


def main():
    have_cargo()
    ctx = BenchContext()

    # print table of results
    print(f"File                          | C      Rust   Delta")
    try:
        for f in ctx.input_files:
            c = benchmark_c_code(ctx, f)
            r = benchmark_rs_code(ctx, f)
            print(f"{os.path.basename(f):30}| {c:0.4f} {r:0.4f} {(c/r - 1.0) * 100:0.2f}%")
    except KeyboardInterrupt:
        print("\nInterrupted. Terminating.")    
        exit(1)
    

if __name__ == "__main__":
    main()


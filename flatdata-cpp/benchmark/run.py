#!/usr/bin/python

import subprocess
import re
import pandas as pd

expr_cpu_time = re.compile(r'CPU time \(ms\) = ([0-9.]+)')
expr_memory = re.compile(r'Memory rusage peak \(kb\) = ([0-9]+)')

def run(target, graph, nodes, runs):
    cmd = ['./flatdata_benchmark', target, graph, '--num-nodes', str(nodes)]
    if target != 'create':
        cmd += ['--num-runs', str(runs)]
    output = subprocess.check_output(cmd).decode('utf-8')
    try:
        cpu_time = float(expr_cpu_time.search(output).group(1))
        memory_usage = int(expr_memory.search(output).group(1))
    except Exception as e:
        print("{} in output {}".format(e, output))
        return None
    return {
        'target':target,
        'graph':graph,
        'nodes':nodes,
        'cpu_time(s)':cpu_time,
        'peak mem(kb)':memory_usage
    }

if __name__ == "__main__":
    GRAPHS = ['struct', 'flatdata']
    NUM_NODES = 20000000
    NUM_RUNS = 10

    results = []
    for target in ['create', 'lookup', 'dijkstra', 'bfs']:
        for graph_name in GRAPHS:
            print("Executing {} on {}".format(target, graph_name))
            result = run(target, graph_name, NUM_NODES, NUM_RUNS)
            results.append(result)

    print(pd.DataFrame(results).to_string())

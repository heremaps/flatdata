#!/usr/bin/python

import subprocess
import re
import pandas as pd

expr_cpu_time = re.compile(r'CPU time \(ms\) = ([0-9.]+)')
expr_memory = re.compile(r'Memory rusage peak \(kb\) = ([0-9]+)')

def run(target, graph, nodes, runs):
    output = subprocess.check_output(['./flatdata_benchmark', target, graph, '--num_runs', str(runs), '--num_nodes', str(nodes)])
    try:
        cpu_time = float(expr_cpu_time.search(output).group(1))
        memory_usage = int(expr_memory.search(output).group(1))
    except:
        print output
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
            result = run(target, graph_name, NUM_NODES, NUM_RUNS)
            print result
            results.append(result)

    print pd.DataFrame(results).to_string()

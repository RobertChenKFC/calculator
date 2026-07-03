#!/usr/bin/env python3

import abc
import ast
import argparse
import json
import os
import re
import shutil
import subprocess


NEG_INF = -1


class Port:
    def __hash__(self):
        return hash(self.to_tuple())

    def __eq__(self, other):
        return self.to_tuple() == other.to_tuple()

    def __lt__(self, other):
        t0 = self.to_tuple()
        t1 = other.to_tuple()
        if t0[0] == t1[0]:
            return t0[1:] < t1[1:]
        return t0[0] == OuterPort


class OuterPort(Port):
    def __init__(self, name):
        self.name = name

    def __str__(self):
        return f"OuterPort(name={self.name})"

    def to_tuple(self):
        return (OuterPort, self.name)

    __repr__ = __str__


class InnerPort(Port):
    def __init__(self, instance_name, instance_module, port_name):
        self.instance_name = instance_name
        self.instance_module = instance_module
        self.port_name = port_name

    def __str__(self):
        return (
            f"InnerPort(instance_name={self.instance_name}, "
            f"instance_module={self.instance_module}, "
            f"port_name={self.port_name})"
        )

    __repr__ = __str__

    def to_tuple(self):
        return (
            InnerPort, self.instance_name, self.instance_module, self.port_name
        )


class Wire:
    def __init__(self):
        self.srcs = set()
        self.dsts = set()

    def add_src(self, src):
        self.srcs.add(src)

    def add_dst(self, dst):
        self.dsts.add(dst)

    def __str__(self):
        return f"Wire(srcs={self.srcs}, dsts={self.dsts})"

    __repr__ = __str__


class Graph(metaclass=abc.ABCMeta):
    def __init__(self, map):
        self.map = map

    def dsts(self):
        return self.map.keys()

    @abc.abstractmethod
    def srcs(self, dst):
        pass

    def __str__(self):
        return str(self.map)


class UnweightedGraph(Graph):
    def srcs(self, dst):
        if dst not in self.map:
            return []
        return self.map[dst]


class WeightedGraph(Graph):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.order = None
        self.roots = None

    def srcs(self, dst):
        if dst not in self.map:
            return []
        return [src for (_, src) in self.map[dst]]

    def weight_src_pairs(self, dst):
        if dst not in self.map:
            return []
        return self.map[dst]

    def freeze(self):
        if self.order is not None:
            return
        self.order = compute_topological_order(self)
        self.roots = set()
        for dst in self.map:
            self.roots.update(self.srcs(dst))
        for dst in self.map:
            self.roots.discard(dst)


    def longest_path_from_src(self, src):
        self.freeze()
        assert self.order is not None

        dist = dict()
        for root in self.roots:
            dist[root] = NEG_INF
        dist[src] = 0

        parents = dict()
        for cur_dst in self.order:
            max_d = NEG_INF
            for (d, cur_src) in self.weight_src_pairs(cur_dst):
                if cur_src not in dist or dist[cur_src] == NEG_INF:
                    continue
                cur_d = dist[cur_src] + d
                if cur_d > max_d:
                    max_d = cur_d
                    parents[cur_dst] = cur_src

            if max_d != NEG_INF:
                dist[cur_dst] = max_d

        return dist, parents

    def longest_paths(self):
        self.freeze()
        dists = dict()

        parents_for_src = dict()
        for src in self.roots:
            dist, parents = self.longest_path_from_src(src)
            parents_for_src[src] = parents
            for dst, d in dist.items():
                if d == NEG_INF or src == dst:
                    continue
                dists.setdefault(dst, [])
                dists[dst].append((d, src))
        return WeightedGraph(dists), parents_for_src


class Module:
    def __init__(self, module_name, module, converted_modules):
        self.wires = dict()
        self.deps = dict()
        self.populate_ports(module)
        if is_74_series_ic(module_name):
            self.compute_deps_for_74_series_ic(module["attributes"])
        else:
            self.add_ports(module["ports"])
            self.add_cells(module["cells"])
            self.convert_to_deps(module["cells"], converted_modules)
        self.deps = WeightedGraph(self.deps)
        self.compute_delays()

    def compute_deps_for_74_series_ic(self, attrs):
        assert "groups" in attrs
        assert "delay" in attrs
        groups = ast.literal_eval(attrs["groups"])
        delay = parse_delay(attrs["delay"])
        for srcs, dsts in groups:
            for src in srcs:
                for dst in dsts:
                    src_port = OuterPort(f"{src}[0]")
                    dst_port = OuterPort(f"{dst}[0]")
                    self.deps.setdefault(dst_port, set())
                    self.deps[dst_port].add((delay, src_port))

    def populate_ports(self, module):
        self.input_ports = []
        self.output_ports = []
        ports = module["ports"]
        for port_name, port in ports.items():
            dir = port["direction"]
            for i, _ in enumerate(port["bits"]):
                outer_port = OuterPort(f"{port_name}[{i}]")
                if dir == "input":
                    self.input_ports.append(outer_port)
                elif dir == "output":
                    self.output_ports.append(outer_port)
                else:
                    raise ValueError(f"Unexpected port direction {dir}")

    def add_node_based_on_dir(self, dir, bit, if_input, if_output, node):
        if dir == "input":
            attr = if_input
        elif dir == "output":
            attr = if_output
        else:
            raise ValueError(f"Unexpected port direction {dir}")
        self.wires.setdefault(bit, Wire());
        getattr(self.wires[bit], attr)(node)

    def add_ports(self, ports):
        for port_name, port in ports.items():
            dir = port["direction"]
            for i, bit in enumerate(port["bits"]):
                self.add_node_based_on_dir(
                    dir, bit, if_input="add_src", if_output="add_dst",
                    node=OuterPort(f"{port_name}[{i}]")
                )

    def add_cells(self, cells):
        for instance_name, instance in cells.items():
            instance_module = instance["type"]
            dirs = instance["port_directions"]
            connections = instance["connections"]
            for port_name, dir in dirs.items():
                port_connections = connections[port_name]
                for i, bit in enumerate(port_connections):
                    self.add_node_based_on_dir(
                        dir, bit, if_input="add_dst", if_output="add_src",
                        node=InnerPort(
                            instance_name, instance_module, f"{port_name}[{i}]"
                        )
                    )

    def convert_to_deps(self, cells, converted_modules):
        # Add the shared-wire connections.
        for wire in self.wires.values():
            for dst in wire.dsts:
                self.deps.setdefault(dst, set())
                for src in wire.srcs:
                    self.deps[dst].add((0, src))
        # Add the dependencies from output ports to input ports of each
        # instantiated module.
        for instance_name, instance in cells.items():
            instance_module_name = instance["type"]
            instance_module = converted_modules[instance_module_name]
            for output_port in instance_module.delays.dsts():
                weight_src_pairs = instance_module.delays.weight_src_pairs(
                    output_port
                )
                output_port = InnerPort(
                    instance_name, instance_module_name, output_port.name
                )
                self.deps.setdefault(output_port, set())
                for (
                    delay, input_port
                ) in weight_src_pairs:
                    input_port = InnerPort(
                        instance_name, instance_module_name, input_port.name
                    )
                    self.deps[output_port].add((delay, input_port))

    def compute_delays(self):
        self.delays = dict()
        longest_paths, parents_for_src = self.deps.longest_paths()
        for dst in longest_paths.dsts():
            if not isinstance(dst, OuterPort):
                continue
            for d, src in longest_paths.weight_src_pairs(dst):
                if not isinstance(src, OuterPort):
                    continue
                self.delays.setdefault(dst, [])
                self.delays[dst].append((d, src))
        self.delays = WeightedGraph(self.delays)
        self.parents_for_src = parents_for_src



    def __str__(self):
        return str(self.delays)


class ModuleResults:
    def __init__(self, name, delay, longest_path):
        self.name = name
        self.delay = delay
        self.longest_path = longest_path


def is_74_series_ic(module_name):
    return re.match(r"ic_74ac(\d)+", module_name) is not None


def parse_args():
    parser = argparse.ArgumentParser(
        description="Compute the critical path of a module given its verilog" \
        "implementation"
    )
    parser.add_argument("module_file", help="Path to verilog module file")
    return parser.parse_args()


def get_module_name(module_file):
    return os.path.splitext(os.path.split(module_file)[-1])[0]


def generate_graph_json(module_file, module_name):
    yosys = shutil.which("yosys")
    script = (
        f"read_verilog {module_file}; hierarchy -top {module_name}; proc; "
        "write_json -"
    )
    run = subprocess.run(
        [yosys, "-q", "-p", script], capture_output=True, text=True,
    )
    if run.returncode != 0:
        print(f"Yosys error: {run.stderr}")
        exit(1)
    output = json.loads(run.stdout)
    return output


def convert_modules(module_order, graph_json):
    converted_modules = dict()
    modules = graph_json["modules"]
    for module_name in module_order:
        module = modules[module_name]
        converted_module = Module(module_name, module, converted_modules)
        converted_modules[module_name] = converted_module
    return converted_modules


def compute_topological_order(graph):
    VISITING = 0
    VISITED = 1
    ADDED = 2

    root_nodes = set(graph.dsts())
    for dst in list(root_nodes): 
        root_nodes.difference_update(graph.srcs(dst))

    stack = list(root_nodes)
    states = { node: VISITING for node in root_nodes }
    order = []
    while stack:
        dst = stack.pop()
        dst_state = states[dst]
        if dst_state == VISITING:
            stack.append(dst)
            states[dst] = VISITED
            for src in graph.srcs(dst):
                if src not in states:
                    states[src] = VISITING
                stack.append(src)
                # We have reached a state that we are have visited all children,
                # so there is a cycle, thus there is no topological order.
                if states[src] == VISITED:
                    return None
        elif dst_state == VISITED:
            order.append(dst)
            states[dst] = ADDED
        else:
            assert dst_state == ADDED
    return order


def test_compute_topological_order():
    class TestCase:
        def __init__(self, graph, is_dag):
            self.graph = UnweightedGraph(graph)
            self.is_dag = is_dag

        def run(self):
            order = compute_topological_order(self.graph)
            if self.is_dag:
                assert len(set(order)) == len(order)
                order_map = {
                    node: i for i, node in enumerate(order)
                }
                for dst in self.graph.dsts():
                    for src in self.graph.srcs(dst):
                        assert order_map[src] < order_map[dst]
            else:
                assert order == None
    test_cases = [
        TestCase(graph={
            "e": {"a", "c", "d"},
            "d": {"a", "b", "c"},
            "c": {"a"},
            "b": {"a"},
            "a": {}
        }, is_dag=True),
        TestCase(graph={
            "e": {"a", "c", "d"},
            "d": {"a", "b", "c"},
            "c": {"a"},
            "b": {"a"},
        }, is_dag=True),
        TestCase(graph={
            "b": {"a"},
            "c": {"b"},
            "d": {"c"},
            "a": {"d"},
            "e": {"d"}
        }, is_dag=False)
    ]
    for test_case in test_cases:
        test_case.run()


def extract_module_results(module_name, module):
    delay = 0
    for dst in module.delays.dsts():
        for d, src in module.delays.weight_src_pairs(dst):
            if d > delay:
                delay = d
                furthest_pair = (src, dst)

    src, dst = furthest_pair
    parent = module.parents_for_src[src]
    longest_path = [dst]
    cur = dst
    while cur != src:
        cur = parent[cur]
        longest_path.append(cur)
    longest_path = reversed(longest_path)

    return ModuleResults(module_name, delay, longest_path)


def report_module_results(module_results):
    ns_delay = module_results.delay // 10
    sub_ns_delay = module_results.delay % 10
    print(
        f"Propagation delay for {module_results.name}: "
        f"{ns_delay}.{sub_ns_delay} ns"
    )
    longest_path = '\n-> '.join(str(x) for x in module_results.longest_path)
    print(f"Longest path:\n   {longest_path}")


def get_module_dependency_order(graph_json):
    graph = dict()
    for module_name, module in graph_json["modules"].items():
        if is_74_series_ic(module_name):
            # Treat them as black boxes, do not read the body of these ICs,
            # and they should not have dependencies on other modules
            graph[module_name] = set()
            continue
        graph.setdefault(module_name, set())
        for cell in module["cells"].values():
            graph[module_name].add(cell["type"])
    return compute_topological_order(UnweightedGraph(graph))


def parse_delay(delay_str):
    # Delay is represented in ns. A maximum of 1 decimal place is allowed.
    split_delay = delay_str.split(".")
    if len(split_delay) > 1:
        assert len(split_delay) == 2
        integer_part, decimal_part = split_delay
        assert len(decimal_part) == 1
    else:
        integer_part = split_delay[0]
        decimal_part = "0"
    delay = int(integer_part) * 10 + int(decimal_part)
    return delay


def get_module_results(module_file):
    module_name = get_module_name(module_file)
    graph_json = generate_graph_json(module_file, module_name)
    module_order = get_module_dependency_order(graph_json)
    modules = convert_modules(module_order, graph_json)
    module = modules[module_name]
    module_results = extract_module_results(module_name, module)
    return module_results


def main():
    args = parse_args()
    module_results = get_module_results(args.module_file)
    report_module_results(module_results)

    
if __name__ == "__main__":
    main()

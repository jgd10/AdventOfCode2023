from dataclasses import dataclass
from pyvis.network import Network as PVNetwork
import networkx as nx
import matplotlib.pyplot as plt


@dataclass
class Node:
    name: str
    edges: set[str]


@dataclass
class Edge:
    start: str
    end: str


@dataclass
class Network:
    nodes: dict[str, Node]
    #edges: dict[frozenset[str, str], Edge]

    def count_nodes(self, start: str, visited=None) -> int:
        if visited is None:
            visited = set()
        visited.add(start)
        count = 1
        for edge in self.nodes[start].edges:
            if edge not in visited:
                count += self.count_nodes(edge, visited.copy())
        return count



def parse_input():
    with open('./input.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    nodes = {}
    for row in data:
        parent, children = row.split(': ')
        nodes[parent] = Node(parent, {c for c in children.split(' ') if c})
    nodes_to_be_added = {}
    for value in nodes.values():
        for edge in value.edges:
            if edge not in nodes:
                if edge not in nodes_to_be_added:
                    nodes_to_be_added[edge] = Node(edge, {value.name})
                else:
                    nodes_to_be_added[edge].edges.add(value.name)
    nodes = {**nodes, **nodes_to_be_added}
    return Network(nodes)


def visualise(network):
    net = PVNetwork()
    for k, v in network.nodes.items():
        net.add_node(k)
    for k, v in network.nodes.items():
        for e in v.edges:
            net.add_edge(k, e)
    net.toggle_physics(True)
    net.show('mygraph.html', notebook=False)


def main(visualise=False):
    network = parse_input()
    if visualise:
        visualise(network)
    edges_to_cut = {'tbg': 'ljh', 'mfs': 'ffv', 'qnv': 'mnh', 'ljh': 'tbg', 'ffv': 'mfs', 'mnh': 'qnv'}
    for k, node in network.nodes.items():
        if k in edges_to_cut:
            if edges_to_cut[k] in node.edges:
                node.edges.remove(edges_to_cut[k])
    graph = nx.Graph()
    for name, node in network.nodes.items():
        graph.add_node(name)
    for node in network.nodes.values():
        for edge in node.edges:
            graph.add_edge(node.name, edge)
    graphs = [graph.subgraph(n) for n in nx.connected_components(graph)]
    assert(len(graphs) == 2)
    g1, g2 = graphs
    print(f'Part 1 Answer: {g1.number_of_nodes()*g2.number_of_nodes()}')


if __name__ == '__main__':
    main()
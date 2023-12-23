from dataclasses import dataclass
from collections import namedtuple


Node = namedtuple('Node', ['name', 'edges'])
Edge = namedtuple('Edge', ['start', 'end', 'length'])




@dataclass
class Network:
    nodes: dict[str, Node]

    @classmethod
    def from_file(cls):
        with open('./input2.txt') as f:
            data = [s.strip('\n') for s in f.readlines()]
        nodes = {}
        for row in data:
            node_name, connections = row.split(' - ')
            edges = []
            for connection in connections.split(';'):
                edge_name, weight = connection.split('=')
                edge = Edge(node_name, edge_name, weight)
                edges.append(edge)
            nodes[node_name] = Node(node_name, edges)
        return cls(nodes)


def main():
    network = Network.from_file()
    print(network.nodes)

if __name__ == '__main__':
    main()
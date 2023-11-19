import sys
from io import StringIO
import queue
import json
import networkx as nx
from itertools import chain

parentOffset = 40
nameOffset = 76

def key(k, data):
    return f'{k:x}_{data["db_name"]}_{data["obj_name"]}'

def split_graphs(graph, rootea):
    levels = 2
    d = nx.bfs_layers(graph, [rootea])
    result = []
    rr = [x for i, x in enumerate(d) if i % levels == 0]
    return list(chain.from_iterable(rr))

def tree_depth(graph, rootea):
    d = nx.shortest_path_length(graph, rootea)
    return max([v for k, v in d.items()])

def nx_graph_data(obj_graph, ea, metadata):
    G = nx.DiGraph()
    for k, v in obj_graph.items():
        label = key(k, metadata[k])
        G.add_node(label)
        for el in v:
            l2 = key(el, metadata[el])
            G.add_edge(label, l2)

    print("graph data", G.number_of_nodes(), G.number_of_edges())
    print("subgraph", G.subgraph([ea]))
    # print("tree depth", tree_depth(G, ea))
    print("is_tree", nx.is_tree(G))
    return G

def main():
    p = sys.argv[1] 
    filename = sys.argv[2]
    if p == '':
        print('graph dump file')
        return
    if filename == '':
        print('save files')
        return

    data = {}
    with open(p, 'rb') as f:
        data = json.loads(f.read().decode('utf-8'))
    obj_graph = {int(k): v for k, v in data['graph'].items()}
    ea = data['ea']
    data_info = {int(k): v for k, v in data['data'].items()}
    G = nx_graph_data(obj_graph, ea, data_info)
    nx.write_gml(G, filename)

    # gr = generate_dot_graph_all(obj_graph, ea, data_info)
    # with open(filename, 'wb') as f:
    #     f.write(gr.encode('utf-8'))

    # graphs = split_graph(obj_graph, ea)
    # print('splitted graphs', graphs)
    # for rootea in graphs:
    #     s = generate_dot_subgraph(obj_graph, rootea, data_info, graphs)
    #     if s == '':
    #         continue
    #     with open(filename+f'{rootea}', 'wb') as f:
    #         f.write(s.encode('utf-8'))

if __name__ == '__main__':
    main()
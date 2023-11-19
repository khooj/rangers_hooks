from idaapi import ask_file
import json

parentOffset = 40
nameOffset = 76

def find_root(ea):
    # place cursor at parent offset
    if db.comment(ea) != 'Parent':
        print('cursor should be placed at parent offset with "Parent" comment')
        return

    oldea = ea-parentOffset
    parents = db.x.down(ea)
    while len(parents) != 0:
        ea = parents[0]
        oldea = ea
        ea += parentOffset
        parents = db.x.down(ea)

    ea = oldea
    print('root: ', ea, 'name:', db.name(ea+nameOffset))
    return ea

def build_graph_recursive(ea):
    children = db.x.up(ea)
    result = {}
    print('got children for ea', ea, children)

    for e in children:
        obj_root = e-parentOffset
        subgraph = build_graph(obj_root)
        result[obj_root] = subgraph

    return result

def hexlist(a):
    return list(map(hex, a))

def build_graph_iterative_flat(ea):
    # children = db.x.up(ea)
    # children = [x for x in children if db.comment(x) == 'Parent']
    # children = [x-parentOffset for x in children]

    children = [ea]
    result = {}
    processed = set()

    while len(children) != 0:
        obj_root = children.pop()
        if obj_root in processed:
            continue
        
        c = db.x.up(obj_root)
        # print(f"c1 {obj_root:x} {hexlist(c)}")
        c = [x for x in c if db.comment(x) == 'Parent']
        # print(f"c2 {obj_root:x} {hexlist(c)}")
        c = [x-parentOffset for x in c]
        if len(c) != 0:
            print(f"c3 {obj_root:x} {hexlist(c)}")
        result[obj_root] = c.copy()
        children.extend(c)
        processed.add(obj_root)

    return result

def obj_name(ea):
    return db.name(ea+nameOffset)

def build_data(obj_graph):
    result = {}
    for k in obj_graph:
        result[k] = {
            'db_name': db.name(k),
            'obj_name': db.name(k+nameOffset)
        }
    return result

def main():
    filename = ask_file(1, "*.json", "output")
    if filename == '':
        print("file does not selected")
    ea = find_root(db.here())
    obj_graph = build_graph_iterative_flat(ea)
    assert 0x414634 in obj_graph
    assert 0x440668 in obj_graph
    data_info = build_data(obj_graph)
    print("classes", len(obj_graph))
    with open(filename, 'wb') as f:
        f.write(json.dumps({ 'ea': ea, 'graph': obj_graph, 'data': data_info }).encode('utf-8'))

if __name__ == '__main__':
    main()
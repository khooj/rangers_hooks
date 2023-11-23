import json
import csv

parentOffset = 40
nameOffset = 76
intTableOffset = 4
autoTableOffset = 8
initTableOffset = 12
typeInfoOffset = 16
fieldTableOffset = 20
methodTableOffset = 24
dynTableOffset = 28
instSizeOffset = 36

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

def hexlist(a):
    return list(map(hex, a))

def build_graph_iterative_flat(ea):
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
        # if len(c) != 0:
        #     print(f"c3 {obj_root:x} {hexlist(c)}")
        result[obj_root] = c.copy()
        children.extend(c)
        processed.add(obj_root)

    return result

def obj_name(ea):
    return db.name(ea+nameOffset)

def convBytes(bb):
    a = 0
    for i in reversed(range(len(bb))):
        a = a * 256 + bb[i]
    return a

def build_data(obj_graph):
    result = {}
    for k in obj_graph:
        result[k] = {
            'db_name': db.name(k),
            'obj_name': db.name(k+nameOffset),
            'inttable': convBytes(db.read(k+intTableOffset)),
            'autotable': convBytes(db.read(k+autoTableOffset)),
            'inittable': convBytes(db.read(k+initTableOffset)),
            'typeinfo': convBytes(db.read(k+typeInfoOffset)),
            'fieldtable': convBytes(db.read(k+fieldTableOffset)),
            'methodtable': convBytes(db.read(k+methodTableOffset)),
            'dynamictable': convBytes(db.read(k+dynTableOffset)),
            'size': convBytes(db.read(k+instSizeOffset)),
        }
    return result

# tests
assert convBytes(b'\xC0\x57\x4A\x00') == 4872128

def main():
    from idaapi import ask_file
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

    with open(filename+'.csv', 'w', newline='') as csvfile:
        wr = csv.writer(csvfile, delimiter=',')
        wr.writerow(['name', 'int', 'auto', 'init', 'type', 'field', 'method', 'dyn', 'size'])
        for k, v in data_info.items():
            wr.writerow([
                v['db_name'],
                v['inttable'],
                v['autotable'],
                v['inittable'],
                v['typeinfo'],
                v['fieldtable'],
                v['methodtable'],
                v['dynamictable'],
                v['size'],
            ])

if __name__ == '__main__':
    main()
    # print('a')
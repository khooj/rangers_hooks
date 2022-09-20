checked = set()

def check_func(ea, check_ea, mark_struct):
    try:
        ea_f = db.function.address(ea)
    except:
        return

    if ea_f in checked:
        return
    ops = list(func.iterate(ea_f))

    for i, ea in enumerate(ops):
        op = instruction.ops(ea)
        if instruction.is_call(ea) and op[0] == check_ea:
            op_next = instruction.ops(ops[i+1])
            for i, op1 in enumerate(op_next):
                if isinstance(op1, instruction.intelops.SegmentOffsetBaseIndexScale):
                    if op1.base.name == 'eax':
                        # print(f"Possible player_ptr usage: {hex(ops[i+1])}")
                        try:
                            instruction.op_structure(ops[i+1], i, mark_struct)
                        finally:
                            break

    checked.add(ea_f)


def to_addr(x):
    try:
        return func.address(x)
    except:
        return ''

def main():
    print("Start1")
    ll = list(db.functions.iterate("get_player_ptr"))[0]
    ll = func.up(ll)
    uniq = set(ll)
    func1 = function.by("get_player_ptr")
    struct = structure.by("SpaceshipInfo")

    for i, ea in enumerate(uniq):
        check_func(ea, func1.start_ea, struct)
        if i % 100 == 0:
            print(f"Done {i}")


if __name__ == '__main__':
    main()
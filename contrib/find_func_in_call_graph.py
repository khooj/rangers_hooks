# script uses ida-minsc

def check_func(current_func):
    get_player_ptr = function.by("get_player_ptr")
    if get_player_ptr is None:
        print("Can't find function get_player_ptr")
        return

    get_player_ptr_ea = get_player_ptr.start_ea

    found_calls = []
    func_instr = list(func.iterate(current_func))
    for i, ea in enumerate(func_instr):
        is_call = instruction.type.is_call(ea) 
        op = instruction.ops(ea)
        if is_call:
            is_equal = op[0] == get_player_ptr_ea
            # print(f"is_call {is_call} and is_equal {is_equal} at {hex(ea)} with op {hex(op[0])} eq {hex(get_player_ptr_ea)}")

            if is_equal:
                # print(f"Found get_player_ptr call in current function at addr {hex(ea)}")
                found_calls.append((i, ea))

    calls_to_check = []
    for st, ea in found_calls:
        # print(f"Searching for given call {hex(ea)}")
        found = False
        found_ea = 0
        for st_i in range(st+1, len(func_instr)):
            ea2 = func_instr[st_i]
            mnem = instruction.mnemonic(ea2)
            if mnem in ['call']:
                found_ea = ea2
                found = True
                break

            if mnem == 'mov':
                op = instruction.op(ea2, 0)
                if isinstance(op, register_t) and op.name == 'eax':
                    # print(f"Probably call get_player_ptr at {hex(ea)} wouldn't give needed result because of override at {hex(ea2)}")
                    break

        if not found:
            # print(f"Can't find next instruction for associated call at {hex(ea)}")
            continue

        op = instruction.ops(found_ea)
        if op[0] == get_player_ptr_ea:
            continue

        calls_to_check.append((ea, op[0]))
        # print(f"Found appropriate call at {hex(found_ea)} op {hex(op[0])}")

    planet_dst_func = function.by("maybe_set_player_planet_destination")
    if planet_dst_func is None:
        print("Can't find destination func")
        return

    for call_ea, func_ea in calls_to_check:
        result = check_call_graph_for_func(func_ea, planet_dst_func.start_ea)
        if len(result) > 0:
            print(f"Found this calls in call graph of {hex(func_ea)} at {hex(call_ea)}")
            for r in result:
                print(f"\tcalled at {hex(r)}")

def check_call_graph_for_func(func_ea, check_ea):
    if not isinstance(func_ea, int):
        return []
    if not isinstance(instruction.op(func_ea, 0), int):
        return []

    result = []
    calls = func.down(func_ea)
    checked = set()
    while len(calls) > 0:
        current_ea = calls.pop()
        if current_ea in checked:
            continue

        mnem = instruction.mnemonic(current_ea)
        if mnem != "call":
            continue

        op_n = instruction.op(current_ea, 0)
        if not isinstance(op_n, int):
            continue

        try:
            calls.extend(func.down(current_ea))
        except Exception as e:
            # print(f"Probably {hex(current_ea)} is not function")
            continue

        for ea in func.iterate(current_ea):
            ops = instruction.ops(ea)
            if instruction.type.is_call(ea) and isinstance(ops[0], int) and ops[0] == check_ea:
                result.append(ea)

        checked.add(current_ea)

    return result

def to_addr(x):
    try:
        return func.address(x)
    except:
        return ''

def main():
    from itertools import chain

    print("Start script")
    ll = list(db.functions.iterate("get_player_ptr"))
    ll = func.up(ll[0])
    ll = map(to_addr, ll)
    ll = filter(lambda x: x != '', ll)

    uniq1 = set(ll)
    for ea in uniq1:
        print(f"Checking func at {hex(ea)}")
        check_func(ea)

if __name__ == '__main__':
    main()
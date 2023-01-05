# script uses ida-minsc

checked = set()
checked_dict = {}

def check_call_graph_for_func(func_ea, check_ea, visited):
    if not isinstance(func_ea, int):
        return ""

    if func_ea in checked:
        return checked_dict[func_ea]

    if func_ea in visited:
        return f"breaked recursive call to {hex(func_ea)}"

    visited.add(func_ea)

    try:
        calls = func.down(func_ea)
    except:
        checked.add(func_ea)
        checked_dict[func_ea] = ""
        return ""

    if check_ea in calls:
        ret = f"{hex(func_ea)} -> {hex(check_ea)}"
        checked.add(func_ea)
        checked_dict[func_ea] = ret
        return ret

    for ea in calls:
        if ea in visited:
            continue
        result = check_call_graph_for_func(ea, check_ea, visited)
        if result != "":
            checked.add(func_ea)
            checked_dict[func_ea] = f"{hex(ea)} -> {result}"
            return checked_dict[func_ea]
    
    checked.add(func_ea)
    checked_dict[func_ea] = ""
    return ""

def to_addr(x):
    try:
        return func.address(x)
    except:
        return ''

def main():
    from itertools import chain

    planet_dst_func = function.by("maybe_set_player_planet_destination")
    if planet_dst_func is None:
        print("Can't find destination func")
        return

    print("Start script")
    ll = list(db.functions.iterate("get_player_ptr"))
    ll = func.up(ll[0])
    ll = map(to_addr, ll)
    ll = filter(lambda x: x != '', ll)

    uniq1 = set(ll)
    results = {}
    for i, ea in enumerate(uniq1):
        if i % 10 == 0:
            print(f"Did {i}")
        print(f"Checking func at {hex(ea)}")
        result = check_call_graph_for_func(ea, planet_dst_func.start_ea, set())
        if result != "":
            results[ea] = result

    print("Found results:")
    for k in results:
        print(f"{hex(k)} => {results[k]}")

if __name__ == '__main__':
    main()
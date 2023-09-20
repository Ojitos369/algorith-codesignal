"""
Given an array of strings, return another array containing all of its longest strings.

Example

For inputArray = ["aba", "aa", "ad", "vcd", "aba"], the output should be
solution(inputArray) = ["aba", "vcd", "aba"].
"""

def solution(d):
    print()
    print(d)
    ml = max([len(s) for s in d])
    rs = [s for s in d if len(s) == ml]
    return rs
    for r in d:
        lr = len(r)
        if lr > max:
            max = lr
            rs = []
        if lr == max:
            rs.append(r)

    if len(d) < 2:
        return d
    elif len(d) == 2:
        return [d[0]] if len(d[0]) > len(d[1]) else ([d[1]] if len(d[1]) > len(d[0]) else d)
    else:
        r = solution(d[1:])
        return [d[0]] if len(d[0]) > len(r[0]) else ([*r] if len(r[0]) > len(d[0]) else [d[0], *r])

    # return rs


def main():
    data = ["aba", "aa", "ad", "vcd", "aba"]
    r = solution(data)
    sol = ["aba", "vcd", "aba"]
    print(f"{sol} -> {r} \t {sol == r}")


def test():
    pass


if __name__ == '__main__':
    main()
    # test()
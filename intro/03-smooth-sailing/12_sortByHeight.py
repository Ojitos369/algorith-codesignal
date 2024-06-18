"""
Description of problem
"""

def solution_new(d): # d of data
    fj = d.count(-1)
    l = sorted([*d])
    print(l, fj)
    r = [-1 if n == -1 else l[fj-i] for i,n in enumerate(d)]
    return r

def solution(d): # d of data
    print()
    print(d)
    l = [*d]
    fj = l.count(-1)
    l = sorted(l)[fj:]
    r = []
    i = 0
    for n in d:
        if n == -1:
            r.append(-1)
        else:
            r.append(l[i])
            i += 1
    return r


def main():
    tests = [
        [-1, 150, 190, 170, -1, -1, 160, 180],
    ]
    sols = [
        [-1, 150, 160, 170, -1, -1, 180, 190],
    ]
    for i in range(len(tests)):
        data = tests[i]
        r = solution(data)
        print(f"{sols[i]} -> {r} \t {sols[i] == r}")


def test():
    pass


if __name__ == '__main__':
    main()
    # test()
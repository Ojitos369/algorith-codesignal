"""
Given a sequence of integers as an array, determine whether it is possible to obtain a strictly increasing sequence by removing no more than one element from the array.

Note: sequence a0, a1, ..., an is considered to be a strictly increasing if a0 < a1 < ... < an. Sequence containing only one element is also considered to be strictly increasing.

Example

For sequence = [1, 3, 2, 1], the output should be
solution(sequence) = false.

There is no one element in this array that can be removed in order to get a strictly increasing sequence.

For sequence = [1, 3, 2], the output should be
solution(sequence) = true.

You can remove 3 from the array to get the strictly increasing sequence [1, 2]. Alternately, you can remove 2 to get the strictly increasing sequence [1, 3].
"""


def solution(l):
    print()
    vals = []
    ls = list(set(l))
    s = sorted(ls)
    sstr = str(s)
    print(l, s)
    def val(i):
        tl = l[:i] + l[i+1:]
        return sorted(set(tl)) == tl
    return any(( val(i) for i in range(len(l)))) if len(l) - len(ls) <= 1 else False


def main():
    data = [1, 3, 2, 1]
    r = solution(data)
    sol = False
    print(f"{sol} -> {r} \t {sol == r}")

    data = [1, 3, 2]
    r = solution(data)
    sol = True
    print(f"{sol} -> {r} \t {sol == r}")
    
    data = [3, 6, 5, 8, 10, 20, 15]
    r = solution(data)
    sol = False
    print(f"{sol} -> {r} \t {sol == r}")
    
    data = [1, 2, 1, 2]
    r = solution(data)
    sol = False
    print(f"{sol} -> {r} \t {sol == r}")
    
    data = [10, 1, 2, 3, 4, 5]
    r = solution(data)
    sol = True
    print(f"{sol} -> {r} \t {sol == r}")

    data = [0, -2, 5, 6]
    r = solution(data)
    sol = True
    print(f"{sol} -> {r} \t {sol == r}")

    data = [40, 50, 60, 10, 20, 30]
    r = solution(data)
    sol = False
    print(f"{sol} -> {r} \t {sol == r}")
    
    data = [1, 1]
    r = solution(data)
    sol = True
    print(f"{sol} -> {r} \t {sol == r}")

    data = [1, 2, 3, 4, 3, 6]
    r = solution(data)
    sol = True
    print(f"{sol} -> {r} \t {sol == r}")
    
    data = [10, 1, 2, 3, 4, 5, 6, 1]
    r = solution(data)
    sol = False
    print(f"{sol} -> {r} \t {sol == r}")
    

def test():
    a = [1,2,3,4,5]
    la = len(a)
    for i in range(la):
        iz = (i-1) % la
        cn = i % la
        de = (i+1) % la
        print(a[iz], a[cn], a[de])

if __name__ == '__main__':
    main()
    # test()
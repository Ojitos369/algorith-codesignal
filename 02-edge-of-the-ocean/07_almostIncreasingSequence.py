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
    print(l)
    if len(l) < 3:
        return True
    errores = 0
    errores_rep = 0
    # if l[0] >= l[-1]:
    #     errores += 1
    # errors = []
    for i in range(len(l) - 1):
        if l[i] >= l[i+1]:
            errores += 1
        if i > 0:
            if l[i-1] >= l[i+1]:
                errores_rep += 1
        if errores > 1 or errores_rep > 1:
            return False
    return True

def solution_ant(l):
    if len(l) - len(set(l)) > 1:
        return False
    def val(i):
        tm = l[:i] + l[i+1:]
        return sorted(set(tm)) == tm
    return any(( val(i) for i in range(len(l))))
    

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

    data = [1, 1, 1, 2, 3]
    r = solution(data)
    sol = False
    print(f"{sol} -> {r} \t {sol == r}")

    data = [1, 1, 1, 2, 3]
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
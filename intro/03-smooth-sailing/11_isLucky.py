"""
Ticket numbers usually consist of an even number of digits. A ticket number is considered lucky if the sum of the first half of the digits is equal to the sum of the second half.
"""

def solution(d): # d of data
    print()
    print(d)
    d = [int(n) for n in str(d)]
    ld = len(d)
    md = ld//2
    return sum(d[0:md]) == sum(d[md:ld])


def main():
    tests = [
        1230, 
        239017
    ]
    sols = [
        True,
        False
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
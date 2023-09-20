"""
Given two strings, find the number of common characters between them.

Example

For s1 = "aabcc" and s2 = "adcaa", the output should be
solution(s1, s2) = 3.

Strings have 3 common characters - 2 "a"s and 1 "c".
"""

def solution(s1, s2): # d of data
    print()
    print(s1, s2)
    checked = []
    def add(c, c1, c2):
        nonlocal checked
        checked.append(c)
        return c1 if c1 - c2 <= 0 else c2
    return sum( add(c, s1.count(c), s2.count(c)) if c not in checked else 0 for c in s1)


def main():
    tests = [
        ["aabcc", "adcaa"],
        ["zzzz", "zzzzzzz"],
        ["abca", "xyzbac"],
    ]
    sols = [
        3, 4, 3
    ]
    for i in range(len(tests)):
        data = tests[i]
        r = solution(*data)
        print(f"{sols[i]} -> {r} \t {sols[i] == r}")


def test():
    pass


if __name__ == '__main__':
    main()
    # test()
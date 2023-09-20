

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
    return sum([int(s1[i] == s2[i]) for i in range(len(s1))])


def main():
    data = ["aabcc", "adcaa"]
    r = solution(*data)
    sol = 3
    print(f"{sol} -> {r} \t {sol == r}")


def test():
    pass


if __name__ == '__main__':
    main()
    # test()
"""
You are given a two-digit integer n. Return the sum of its digits.

Example

For n = 29, the output should be
solution(n) = 11
"""

def solution(d):
    print()
    print(d)
    return sum(int(n) for n in str(d))


def main():
    data = 29
    r = solution(data)
    sol = 11
    print(f"{sol} -> {r} \t {sol == r}")


def test():
    pass


if __name__ == '__main__':
    main()
    # test()
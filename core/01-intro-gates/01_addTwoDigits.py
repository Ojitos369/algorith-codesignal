"""
You are given a two-digit integer n. Return the sum of its digits.

Example

For n = 29, the output should be
solution(n) = 11
"""

def solution(n):
    print()
    print(n)
    return sum(int(d) for d in str(n))


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
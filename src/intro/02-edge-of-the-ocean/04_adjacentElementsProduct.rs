"""
Given an array of integers, find the pair of adjacent elements that has the largest product and return that product.

Example

For inputArray = [3, 6, -2, -5, 7, 3], the output should be
solution(inputArray) = 21.

7 and 3 produce the largest product.
"""


def solution(l):
    return max(l[0]*l[1], solution(l[1:])) if len(l) > 2 else l[0]*l[1]


if __name__ == '__main__':
    inputArray = [3, 6, -2, -5, 7, 3]
    r = solution(inputArray)
    print(r)
"""
Write a function that returns the sum of two numbers.

Example

For param1 = 1 and param2 = 2, the output should be
solution(param1, param2) = 3.
"""

# def solution(param1, param2):
#     return param1 + param2

def solution(*args):
    return sum(args)


if __name__ == '__main__':
    param1 = 1
    param2 = 2
    r = solution(param1, param2)
    print(r)
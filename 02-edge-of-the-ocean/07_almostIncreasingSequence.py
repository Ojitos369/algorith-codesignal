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
    err = 0
    for i in range(len(l) - 1):
        copy = l.copy()
        copy.pop(i)
        unique = sorted(list(set(copy)))
        if unique != copy:
            err += 1
            if err > 1:
                return False
    return True
        
                


if __name__ == '__main__':
    data = [1, 3, 2, 1]
    print(solution(data))

    data = [1, 3, 2]
    print(solution(data))
    
    data = [1, 2, 1, 2]
    print(solution(data))
    
    data = [10, 1, 2, 3, 4, 5]
    print(solution(data))
    
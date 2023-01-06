"""
Ratiorg got statues of different sizes as a present from CodeMaster for his birthday, each statue having an non-negative integer size. Since he likes to make things perfect, he wants to arrange them from smallest to largest so that each statue will be bigger than the previous one exactly by 1. He may need some additional statues to be able to accomplish that. Help him figure out the minimum number of additional statues needed.

Example

For statues = [6, 2, 3, 8], the output should be
solution(statues) = 3.

Ratiorg needs statues of sizes 4, 5 and 7.
"""


def solution(l):
    l = sorted(l)
    needed = 0
    i = 0
    while i < len(l) - 1:
        diff = l[i+1] - l[i]
        if diff > 1:
            needed += (diff - 1)
        i += 1
    return needed


if __name__ == '__main__':
    data = [6, 2, 3, 8]
    print(solution(data))
    
    data = [0,3]
    print(solution(data))
    
    data = [1,2,3,8,9,5,4,7,4,5,6]
    print(solution(data))
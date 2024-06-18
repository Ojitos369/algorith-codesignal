"""
Description of problem
"""

def solution(d): # d of data
    print()
    print(d)
    ...
    return "correct_answer"


def main():
    tests = [
        "input_data", 
        ...
    ]
    sols = [
        "correct_answer",
        ...
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
"""
Description of problem
"""

def solution(d): # d of data
    print()
    print(d)
    ...
    return "correct_answer"


def main():
    data = "input_data"
    r = solution(data)
    sol = "correct_answer"
    print(f"{sol} -> {r} \t {sol == r}")


def test():
    pass


if __name__ == '__main__':
    main()
    # test()
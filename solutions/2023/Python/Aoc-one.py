import string

def find_digits_first_last(s):
    digits = [l for l in s if l in string.digits]
    return int("{}{}".format(digits[0], digits[-1]))

cal_vals = []

with open("pi", "r") as f:
    content = f.read().split("\n")
    for line in content : 
        cal_val = find_digits_first_last(line)
        cal_vals.append(cal_val)

print(sum(cal_vals))
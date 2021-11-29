import hashlib

INPUT = "bgvyzdsv"
# change pattern to 000000 for part 2
STARTING_PATTERN = "00000" 
i = 0
while 1:
    if hashlib.md5(f"{INPUT}{i}".encode("utf-8")).hexdigest().startswith(STARTING_PATTERN):
        print(i)
        break
    i += 1

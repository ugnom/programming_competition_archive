words = lambda t : list(map(t, input().split()))
n = int(input())
a = words(int)

b = [0] * (n+1)
b[0] = 3

ans = 1
# 0 0 1 0 1 2 1 2 3 2 3 3 4 4 5 4 6 5 7 8 5 6 6 7 7 8 8 9 9 10 10 11 9 12 10 13 14 11 11 12 12 13 13 14 14 15 15 15 16 16 16 17 17 17
for ai in a:
    ans *= b[ai]
    ans %= 1000000007
    b[ai] -= 1
    b[ai+1] += 1

print(ans)

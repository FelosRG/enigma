import numpy as np

N = 84

assert N % 2 == 0 , "El número de caracteres N debe de ser par!"

def big_print(s):
    n = len(s)
    print("\n" + "-"*n)
    print(s)
    print("-"*n + "\n")


big_print("Rotor 1")
pos = np.arange(N)
np.random.shuffle(pos)
print(pos)

big_print("Rotor 2")
pos = np.arange(N)
np.random.shuffle(pos)
print(pos)

big_print("Rotor 3")
pos = np.arange(N)
np.random.shuffle(pos)
print(pos)

big_print("Reflector")
pos = np.arange(N)
np.random.shuffle(pos)
n  = int(N / 2) 
r1 = pos[:n]
r2 = pos[n:]

print(r1)
print("\n")
print(r2)
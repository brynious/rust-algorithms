import sys

def get_integers():
    integer_string = str(sys.stdin.readline())
    integers_list = integer_string.split()
    if integers_list == ["0"]:
        return False
    else:
        return integers_list

def euclidean_alg(a,b):
    while b !=0:
        (a,b) = (b, a % b)  #we will divide until we get a GCD with no remainderbetween the two number folling euclid's algorithm for gcd
    return abs(a) #GCD is non-negative

def main():
    integers = get_integers()
    if integers and len(integers) == 1:
        print("The gcd of the integers is ",str(integers[0]),".", sep = "")
        main()
    elif integers:
        start = int(integers[0])
        second = 1
        stop = len(integers)    #we will take the GCD for our first number in the list and compare it with every other number in list
        while second != stop and start != 1:
            start = euclidean_alg(start,int(integers[second])) #We should never get a bigger number in each iteration of obtaining GCD, if we ever reach 1 then we know that there is no other GCD
            second += 1
        print("The gcd of the integers is ", str(start),".", sep= "")
        main()
    else:
        pass

main()

# Deliverable 1

# Deliverable 2

# Deliverable 3
## Question 1
The code randomly generates a number 0 to n. If the number is even then add 1, if the number is odd do nothing. After this, if the number is prime return. If not repeat this process.
## Question 2
Q2 in deliverable3 directory
## Question 3
The Miller Rabin algorithm performs a probabilistic check on if an integer is prime or not, and gains more accuracy the more iterations are used. The code performs a Miller Rabin check with 6 iterations.

1. Fine values s and d where candidate-1 = d*2^s
Repeat 6 times:
2. Generate integer in the range on 2 to candidate-2. Let's call that integer basis
3. Calculate x = basis^d % candidate
4. If isn't x is 1, or x = candidate - 1: continue (start new repitition by going back to step 2)
Otherwise:
For each i in (0 - s-1)
5. calculate y = x^2 % candidate
6. If y = 1, candidate is not prime. If x = candidate - 1, candidate is prime. If neither, continue
7. If i == s - 2, candidate is not prime
8. Start new repition by going back to step 5. If that loop is finished, go back to step 2.
9. If the loop at step 2 completes, the candidate is prime

## Question 4
glass_pumpkin crate generates primes using prime and prejudice. They primes are cryptographically secure (very large)

## Question 5

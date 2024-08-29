# Fast_Powering_Algorithm
This repository contains a Rust implementation of fast modular exponentiation. 
The algorithm efficiently computes a^b mod m,making it highly useful in cryptography and number theory.

Example:
Base (a): 3
Exponent (b): 5
Modulo (modulo): 7

##The goal is to compute 3^5 mod 7.
##Step-by-Step Calculation

    Initial Values:
        res = 1
        base = a % m = 3 % 7 = 3
        b = 5

    First Iteration:
        b = 5 (odd, so update res)
        res = (1 * 3) % 7 = 3
        base = (3 * 3) % 7 = 2
        b = 5 / 2 = 2

    Second Iteration:
        b = 2 (even, so no change to res)
        base = (2 * 2) % 7 = 4
        b = 2 / 2 = 1

    Third Iteration:
        b = 1 (odd, so update res)
        res = (3 * 4) % 7 = 5
        base = (4 * 4) % 7 = 2
        b = 1 / 2 = 0 (loop ends)
##Final Result

After all iterations, res equals 5. 
This means:
    3^5mod  7=5


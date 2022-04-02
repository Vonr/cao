# Documentation

### *-argument operations:

- pi - Adds π to the stack
- tau - Adds τ to the stack
- e - Adds e to the stack
- len - Adds the length of the stack to the stack
- fold - Adds the previous operation enough times to operate on the whole stack (Only useful with 2-argument operations)
- map - Executes the previous operation on each element of the stack
- rev - Reverses the stack

### 1-argument operations:

- \<num\> dup - Duplicates the top element of the stack
- \<num\> pop - Removes the top element of the stack
- \<index\> take - Pushes an element from the pocket at the specified index to the stack
- \<num\> abs - Pushes the absolute value of the top element of the stack
- \<num\> sqrt - Pushes the square root of the top element of the stack
- \<num\> sin - Pushes the sine of the top element of the stack
- \<num\> cos - Pushes the cosine of the top element of the stack
- \<num\> tan - Pushes the tangent of the top element of the stack
- \<num\> asin - Pushes the arcsine of the top element of the stack
- \<num\> acos - Pushes the arccosine of the top element of the stack
- \<num\> atan - Pushes the arctangent of the top element of the stack
- \<num\> sinh - Pushes the hyperbolic sine of the top element of the stack
- \<num\> cosh - Pushes the hyperbolic cosine of the top element of the stack
- \<num\> tanh - Pushes the hyperbolic tangent of the top element of the stack
- \<num\> asinh - Pushes the hyperbolic arcsine of the top element of the stack
- \<num\> acosh - Pushes the hyperbolic arccosine of the top element of the stack
- \<num\> atanh - Pushes the hyperbolic arctangent of the top element of the stack
- \<num\> ln - Pushes the natural logarithm of the top element of the stack
- \<num\> log - Pushes the natural logarithm of the top element of the stack
- \<num\> log2 - Pushes the base-2 logarithm of the top element of the stack
- \<num\> ceil - Pushes the ceiling of the top element of the stack
- \<num\> floor - Pushes the floor of the top element of the stack
- \<num\> round - Pushes the rounded value of the top element of the stack
- \<num\> trunc - Pushes the truncated value of the top element of the stack
- \<int\> fac - Pushes the factorial of the top element of the stack
- \<int\> fib - Pushes the nth Fibonacci number


### 2-argument operations:

- \<num\> \<num\> + - Pushes the sum of the top two elements of the stack
- \<num\> \<num\> - - Pushes the difference of the top two elements of the stack
- \<num\> \<num\> * - Pushes the product of the top two elements of the stack
- \<num\> \<num\> / - Pushes the quotient of the top two elements of the stack
- \<num\> \<num\> ^ - Pushes the power of the top two elements of the stack
- \<num\> \<num\> ** - Pushes the power of the top two elements of the stack
- \<num\> \<num\> mod - Pushes the modulus of the top two elements of the stack
- \<num\> \<num\> % - Pushes the modulus of the top two elements of the stack
- \<num\> \<num\> = - Pushes 1 if the top two elements of the stack are equal, 0 otherwise
- \<num\> \<num\> ~ - Pushes 1 if the top two elements of the stack are approximately equal, 0 otherwise
- \<num\> \<num\> ~= - Pushes 1 if the top two elements of the stack are approximately equal, 0 otherwise
- \<num\> \<num\> != - Pushes 1 if the top two elements of the stack are not equal, 0 otherwise
- \<num\> \<num\> \< - Pushes 1 if the top two elements of the stack are less than, 0 otherwise
- \<num\> \<num\> \<= - Pushes 1 if the top two elements of the stack are less than or equal to, 0 otherwise
- \<num\> \<num\> \> - Pushes 1 if the top two elements of the stack are greater than, 0 otherwise
- \<num\> \<num\> \>= - Pushes 1 if the top two elements of the stack are greater than or equal to, 0 otherwise
- \<num\> \<num\> max - Pushes the maximum of the top two elements of the stack
- \<num\> \<num\> dp - Pushes the truncated decimal representation of the top two elements of the stack
- \<num\> \<index\> store - Stores the top element of the stack at the specified index in the pocket
- \<num\> \<num\> seq - Pushes the sequence of n to m exclusive where n and m are the top two elements of the stack
- \<int\> \<int\> pow - Pushes the first element of the stack raised to the power of the second element of the stack
- \<int\> \<int\> gcd - Pushes the greatest common divisor of the top two elements of the stack
- \<int\> \<int\> \<\< - Pushes the first element of the stack shifted left by the second element of the stack
- \<int\> \<int\> \>\> - Pushes the first element of the stack shifted right by the second element of the stack
- \<int\> \<int\> or - Pushes the bitwise or of the top two elements of the stack
- \<int\> \<int\> and - Pushes the bitwise and of the top two elements of the stack
- \<int\> \<int\> xor - Pushes the bitwise xor of the top two elements of the stack
- \<int\> \<int\> min - Pushes the minimum of the top two elements of the stack
- \<num\> \<num\> over - Pushes the second element of the stack over the top element of the stack
- \<num\> \<num\> swap - Swaps the top two elements of the stack

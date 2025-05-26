# Chapter 5: Elliptic Curves

---

## Summary

---
In this chapter, I've learned:


### **Ex.58** & **Ex.59**

---
These two exercises have their own solution in the example.

### **Ex.60**

---

### **Ex.61**

---
Consider a curve like this: $y^2=x^3+a\cdot x+b$.  
Assume that $x'=c^2\cdot x$ and $y'=c^3\cdot y$. We have $x'^3=c^6\cdot x^3$ and $y'^2=c^6\cdot y^2$.
$(x',y')$ on the curve, so that $y'^2=x'^3+a'\cdot x'+b'$.  
$\Rightarrow c^6\cdot y^2=c^6\cdot x^3 +a\cdot c^4\cdot c^2\cdot x+b\cdot c^6$  
Because $c$ is an invertible field, multiple both side of equation with ${c^{-1}}^6=c^{-6}$:  
$\Rightarrow y^2=x^3 +a\cdot x+b$ is a curve.  

### **Ex.62**

---
$TTJ_{13}: y^2=x^3+8x+8$ and $E_{7,5}(F_{13}): y^2=x^3+7x+5$.
Assume this two curves are isomorphic, that mean $a'=a\cdot c^4$ and $b'=b\cdot c^6$. Base on multiplication table of $13$, we calculate the value of $c^4=\frac{7}{8}=9$ and $c^6=\frac{5}{8}=12$.
Now $c^2=\frac{c^6}{c^4}=10$, and $10$ has square root $\lbrace6,7\rbrace$, that mean there is a possible value for $c$ holds that $a'=a\cdot c^4$ and $b'=b\cdot c^6$.

### **Ex.63**

---
Calculate the inverses:

- $-(10,10)=(10,3)$
- $-\mathcal{O}=\mathcal{O}$
- $-(4,0)=(4,0)$
- $-(1,2)=(1,11)$

Solve the equation: $x\oplus(9,4)=(5,2)$.  

- Plus both side with $-(9,4)=(9,9)$: $x=(5,2)\oplus(9,9)$.
- $x_3=(\frac{y_2-y_1}{x_2-x_1})^2-x_1-x_2=(\frac{9-2}{9-5})^2-5-9=11$
- $y_3=(\frac{y_2-y_1}{x_2-x_1})(x_1-x_3)-y_1=(\frac{9-2}{9-5})(5-11)-9=7$
- $x=(11,7)$

### **Ex.64**

---

$$
\begin{align*}
[1](0, 1) + [1](0, 1) = [2](0, 1) \\
[2](0, 1) + [2](0, 1) = [4](0, 1) \\
[4](0, 1) + [4](0, 1) = [8](0, 1) \\
[8](0, 1) + [8](0, 1) = [7](0, 1) \\
[7](0, 1) + [7](0, 1) = [5](0, 1) \\
[5](0, 1) + [5](0, 1) = [1](0, 1)
\end{align*}
$$

$[3]$, $[6]$, $[9]$ is belonged to logarithmic order. And factorization of 9 is $3\cdot 3$, so $[9]$ has 3 subgroups: 
- A subgroup of order of 9
- A subgroup of order of 3 contains $[3](0, 1) $ and $[6](0, 1) $
- A subgroup of order of 1

### **Ex.65**

--- 
I used my code to compute more easily.Here is my result:
```rust
[10](5, 11) = (0, 1)
[10](9, 4) = (4, 0)
[4](9, 4) = (7, 11)
```

### **Ex.66**

---
This exercise is already solved in the example.

### **Ex.67**

---

See my code [here](./src/lib.rs) to know how I get the result.  
```rust
Logarithm order: [
    Point { x: 7, y: 11, z: 1 }, 
    Point { x: 8, y: 5, z: 1 }, 
    Point { x: 8, y: 8, z: 1 }, 
    Point { x: 7, y: 2, z: 1 }, 
    Point { x: 0, y: 1, z: 0 }]
```

### **Ex.68**

---

See my code [here](./src/lib.rs) to know how I get the result.
```rust
[0, 1, 0] + [4, 3, 1] = Ok(Point { x: 4, y: 3, z: 1 })
[0, 3, 0] + [3, 1, 2] = Ok(Point { x: 0, y: 1, z: 0 })
-[0, 4, 1] + [3, 4, 1] = Ok(Point { x: 3, y: 1, z: 1 })
[4, 3, 1] + [4, 2, 1] = Ok(Point { x: 0, y: 1, z: 0 })
```

### **Ex.69**

---

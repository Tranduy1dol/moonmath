# Chapter 4: Algebra

---
In this chapter, I've learned:

### **Ex.33**

---
Consider all properties of a communicative group on $Z^*_5$, we have:  

- **Commutativity**: for all elements $g_1$, $g_2 \in Z_5^*$, we have $g_1\cdot g_2 = g_2\cdot g_1$.  
- **Associativity**: for all elements  $g_1$, $g_2$, $g_3 \in Z_5^*$, we have
$g_1\cdot (g_2\cdot g_3) =(g_1\cdot g_2)\cdot g_3$.  
- **Existence of a neutral element**: $1$ is the neutral element of $Z_5^*$.  
- **Existence of an inverse**: all elements in $`Z_5^*`$ have inverse.  
So that $`Z_5^*`$ is a communicative group.  

### **Ex.34**

---
With $n$ is a prime number, $(Z_n^*, \cdot)$ is a communicative group with $1$ is the neutral element.

### **Ex.35**

---
The remainder class will have element from $0$ to $n-1$, so the order of $(Z_n, +)$ is $n$.

### **Ex.36**

---
We have $5^1=5$, $5^2=4$, $5^3=3$, $5^4=2$, $5^5=1$, $5^6=0$. We can generate all elements of $`Z^*_6`$ by multiply $5$, so that $5$ is a generator.  
Do the same to $2$, we will have the result that can't generate all elements of $`Z_6^{*}`$ by multiply $2$, so $2$ is not a generator.  

### **Ex.37**

---

### **Ex.38**

---

### **Ex.39**

---
Consider all properties of a communicative group on $Z_5^{*}[2]$, we have:

- Commutativity: Same as $Z_5^*$.
- Associativity: Same as $Z_5^*$.
- Existence of a neutral element: 1 is the neutral element of $Z_5^*[2]$.
- Existence if an inverse: since $4\cdot 4=1$ and $1\cdot 1=1$ over $`Z_5^{*}[2]`$, all elements in $`Z_5^{*}[2]`$ have inverse.
So that $Z_5^*[2]$ is a communicative group.

### **Ex.40**

---
Since $6=3\cdot2\cdot1$, we have all subgroup of $Z_6$ as below:  

- $Z_6[1]=\{0\}$
- $Z_6[2]=\{0, 2\}$
- $Z_6[3]=\{0,2,4\}$
- $Z_6[6]=\{0,1,3,2,4,5\}$

$3$ is the large prime order subgroup of $Z_6$, we get the cofactor clearing map $(+)^2: Z_6\rightarrow Z_6[3]$:  

- $0+0=0,$
- $1+1=2,$
- $2+2=4,$
- $3+3=0,$
- $4+4=2,$
- $5+5=4$

### **Ex.41**

---

### **Ex.42**

---
For $G_1$ we have $g_1, g_1' \in G_1$, for $G_2$ we have $g_2, g_2' \in G_2$.  
If $g_1=g_1'$ we have $e(g_1^a, g_2)=e(g_1,g_2)\cdot e(g_1,g_2)\cdot ...=e(g_1,g_2)^a$.  
If $g_2=g_2'$ we have $e(g_1^a, g_2^b)=e(g_1^a,g_2)\cdot e(g_1^a,g_2)\cdot ...=e(g_1,g_2)^{a\cdot b}$.

### **Ex.43**

---

### **Ex.44**

---
I chose set $\lbrace2, 11, 7\rbrace$ from generators of $Z_{13}=\lbrace2,6,7,11\rbrace$. Construct Pedersen Hash as below:  
$`H_{\lbrace2,7,11\rbrace}:Z_{12}\times Z_{12} \rightarrow Z^{*}_{13}; (x_1,x_2,x_3)\rightarrow2^{x_1}\cdot7^{x_2}\cdot11^{x_3}`$  
With $(3,7,11)\in Z_{12}$, compute the results: $H(3,7,11)=2$

### **Ex.45**

---
Compose the $SHA256$ with the Pedersen Hash from the previous ex., we have the hash function as below:  
$`SHA256\_H_{\lbrace2,7,11\rbrace}:\lbrace0,1\rbrace^{*} \rightarrow Z^{*}_{13}; (s)\rightarrow2^{SHA256(s)_0}\cdot7^{SHA256(s)_1}\cdot11^{SHA256(s)_3}`$

### **Ex.46**

---
Chose seed $\lbrace a_0, a_1, a_2\rbrace=\lbrace 1,2,3,4\rbrace$, generator $2$, the hash compute as below:  
$F(1,0,1)=2^{1\cdot2^{1}\cdot3^{0}\cdot4^{1}}=2^8=9$ over $Z_{13}$

### **Ex.47**

---
In the previous ex., we note that $(Z_5,+)$ , $(Z_5^{*}, \cdot)$ are communicative group. Distributivity also holds for all elements. So that $(Z_5, +, \cdot)$ is a field.  
The characteristic of $(Z_5, +, \cdot)$ is 5, because $1+1+1+1+1=5$, plus 1 five time.  
For each $a\in Z_5$ has only 1 inverse, so that $x=ba^{-1}$ is unique.

### **Ex.48**

---
Not all elements in $Z_6^{*}$ has inverse, so that $(Z_6,+,\cdot)$ is not a field.

### **Ex.49**

---

```
+  0 1 2
 +------
0| 0 1 2
1| 1 2 0
2| 2 0 1

*  0 1 2
 +------
0| 0 0 0
1| 0 1 2
2| 0 2 1
```

### **Ex.50**

---

```
 +   0  1  2  3  4  5  6  7  8  9 10 11 12
  +---------------------------------------
 0|  0  1  2  3  4  5  6  7  8  9 10 11 12
 1|  1  2  3  4  5  6  7  8  9 10 11 12  0
 2|  2  3  4  5  6  7  8  9 10 11 12  0  1
 3|  3  4  5  6  7  8  9 10 11 12  0  1  2
 4|  4  5  6  7  8  9 10 11 12  0  1  2  3
 5|  5  6  7  8  9 10 11 12  0  1  2  3  4
 6|  6  7  8  9 10 11 12  0  1  2  3  4  5
 7|  7  8  9 10 11 12  0  1  2  3  4  5  6
 8|  8  9 10 11 12  0  1  2  3  4  5  6  7
 9|  9 10 11 12  0  1  2  3  4  5  6  7  8
10| 10 11 12  0  1  2  3  4  5  6  7  8  9
11| 11 12  0  1  2  3  4  5  6  7  8  9 10
12| 12  0  1  2  3  4  5  6  7  8  9 10 11

 *   0  1  2  3  4  5  6  7  8  9 10 11 12
  +---------------------------------------
 0|  0  0  0  0  0  0  0  0  0  0  0  0  0
 1|  0  1  2  3  4  5  6  7  8  9 10 11 12
 2|  0  2  4  6  8 10 12  1  3  5  7  9 11
 3|  0  3  6  9 12  2  5  8 11  1  4  7 10
 4|  0  4  8 12  3  7 11  2  6 10  1  5  9
 5|  0  5 10  2  7 12  4  9  1  6 11  3  8
 6|  0  6 12  5 11  4 10  3  9  2  8  1  7
 7|  0  7  1  8  2  9  3 10  4 11  5 12  6
 8|  0  8  3 11  6  1  9  4 12  7  2 10  5
 9|  0  9  5  1 10  6  2 11  7  3 12  8  4
10|  0 10  7  4  1 11  8  5  2 12  9  6  3
11|  0 11  9  7  5  3  1 12 10  8  6  4  2
12|  0 12 11 10  9  8  7  6  5  4  3  2  1
```

### **Ex.51**

---

```
[(0, 1), (0, 12), (1, 0), (2, 4), (2, 9), (4, 2), (4, 11), (5, 6), (5, 7), (6, 5), (6, 8), (7, 5), (7, 8), (8, 6), (8, 7), (9, 2), (9, 11), (11, 4), (11, 9), (12, 0)]
```

3 previous ex. should use code to solve, this is the easiest way. Maybe can use pen-and-paper method but I think it will take a lot of time.

### **Ex.52**

---

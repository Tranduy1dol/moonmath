
# Chapter 6: Statements

---
In this chapter, I've learn:

### **Ex.96**

---
Here I define language $L_{exercise1}$ and its decision function:
$$
R_{Exercise1} : (\mathbb{F}_{13})^* \to \{true, false\} ;
$$
$$
\langle x_1, \ldots, x_n \rangle
\mapsto
\begin{cases}
true & n = 1 \text{ and } 5x_1+4 = 28 + 2x_1 \\
false & else
\end{cases}
$$
By solving this equation in a normal way, I got the solution $x=8$ is a constructive proof, and $R_{exercise1}(\langle8\rangle)=true$ verify this proof.

### **Ex.97**

---
Solving the equation $3x+3=0$ in $`\mathbb{Z}_6`$ , we have the solutions $\langle1\rangle$ and $\langle5\rangle$. Because the decision function require $n=1$ so we have 2 proofs, and $R_{exercise1}(\langle1\rangle)=true$ and $R_{exercise1}(\langle5\rangle)=true$ verify these proof.

### **Ex.98**

---
Solving the equation in each cases, we have:

- $(3,3,0)$ have $1$ as witness (or $5$).
- $(2,1,0)$ have no witness.
- $(4,4,2)$ have 1 as witness.

### **Ex.99**

---
I define instance is composited of 2 points, and witness is 1 point holds that:  
$`\sum_i = \mathbb{F}_{13}`$ and $`\sum_w = \mathbb{F}_{13}`$  
Our grammar define as follow:
$$
R_{add} : (\mathbb{F}_{13})^\ast \times (\mathbb{F}_{13})^\ast \to \{true, false\} ;
$$$$
(i, w)
\mapsto
\begin{cases}
true & (\langle i_1, i_2 \rangle) \in L_{tiny-jj} \ & \text{ and } &(\langle i_3, i_4 \rangle) \in L_{tiny-jj} \ & \text{ and } &(w_1, w_2) = \left( \frac {i_1i_4 + i_2i_3} {1 + 8i_1i_3i_2i_4} , \frac {i_2i_4 - 3i_1i_3} {1 - 8i_1i_3i_2i_4} \right) \\
false & else
\end{cases}
$$
To get the constructive proof that verifiable, choose 2 points on curve as instance and compute their sum as witness. In the other side, choose 1 point outside the curve and 1 point on curve, we will have 1 instance that can't provide a proof.

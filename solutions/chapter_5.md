# Chapter 5: Elliptic Curves

---
In this chapter, I've learned:

### **Ex.58** & **Ex.59**

---
This two exercises have it own solution in the example.

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
$\left[1\right](0,1)+\left[1\right](0,1)=\left[2\right](0,1)$  
$\left[2\right](0,1)+\left[2\right](0,1)=\left[4\right](0,1)$  
$\left[4\right](0,1)+\left[4\right](0,1)=\left[8\right](0,1)$  
$\left[8\right](0,1)+\left[8\right](0,1)=\left[7\right](0,1)$  
$\left[7\right](0,1)+\left[7\right](0,1)=\left[5\right](0,1)$  
$\left[5\right](0,1)+\left[5\right](0,1)=\left[1\right](0,1)$  

$\left[3\right],\left[6\right],\left[9\right]$ is belonged to logarithmic order: $\left[3\right](0,1) \rightarrow \left[6\right](0,1) \rightarrow \mathcal{O}$.

### **Ex.65**

---
$\left[10\right](5,11)=\left[2\times2\times2\right](5,11)+\left[2\right](5,11)$  
$x'=(\frac{3\cdot5^2+8}{2\cdot11})^2-2\cdot5=7,y'=(\frac{3\cdot5^2+8}{2\cdot11})(5-7)-11=11$  
$\Rightarrow\left[2\right](5,11)=(7,11)$  
$x'=(\frac{3\cdot7^2+8}{2\cdot11})^2-2\cdot5=6,y'=(\frac{3\cdot7^2+8}{2\cdot11})(7-6)-11=6$    
$\Rightarrow\left[4\right](5,11)=\left[2\right](7,11)=(6,6)$  
$x'=(\frac{3\cdot6^2+8}{2\cdot6})^2-2\cdot6=2,y'=(\frac{3\cdot6^2+8}{2\cdot6})(6-6)-6=7$    
$\Rightarrow\left[8\right](5,11)=\left[4\right](7,11)=\left[2\right](6,6)=(2,7)$  
$\left[10\right](5,11)=(2,7)\oplus(7,11)$
$x_3=(\frac{11-7}{7-2})^2-2-7=1,y'=(\frac{11-7}{7-2})(2-1)-2=7$    

Do the same way for another computation, here is the results:  
$\left[10\right](9,4)=(4,0), \left[4\right](9,4)=(7,11)$
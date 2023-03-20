## Section 1

Hello World!

\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]

```custom
Hello World!
```

```center
This is a demonstration of the *center* environment. 
This paragraph of text will be *centred* because it is 
contained within a special environment. Environments provide 
an efficient way to modify blocks of text within your document.
```

```boxed "hsl(270, 40%, 94% )" "hsl(270, 40%, 75% )"
This content is inside a block!
```

```definition "Permutative Matrix"
A matrix \\(P\in M_n(\mathbb{C})\\) is called *permutative* or a *permutative matrix* if
\\[
    P = \begin{bmatrix} \pi_1(x)^\top \\\\ \vdots \\\\ \pi_n(x)^\top \end{bmatrix},
\\]
where \\(\pi_1,\dots,\pi_n\in S_n\\) and \\(x\in\mathbb{C}^n\\).
```

## Result

```proposition Suleimanova
Every Suleimanova spectrum is realizable.
```

```lemma
The matrix from the eigenvector of (3) given by
\\[
S = \begin{bmatrix} e & v_2 & v_3 & \dots & v_n \end{bmatrix},
\\]
where \\(n\ge2\\), is invertible whenever \\(x\ge0\\) and \\(x_1\ne\Sigma\\).
```

```custom2
Hello World!
```

```proof
Following the properties of the determinant,
\\[
    |S| = \prod_{i=2}^n(\delta_i - \Sigma).
\\]
Since \\(\Sigma\ne x_1\\) and \\(x_i\ge0\\), \\(\forall i\in\langle n\rangle\\), it follows that \\(\text{det}(S)\ne0\\). Therefore \\(S\\) is not invertible.
```

```theorem
If \\(\Lambda=\\{\lambda_1,\dots,\lambda_n\\}\\) is a list of real numbers, then \\(\Lambda\\) is realizable by a matrix of the form (3) if and only if 
\\[
    s_1(\Lambda) \ge 0
\\]
and
\\[
    s_1(\Lambda) - n\lambda_i\ge 0,~i\in\langle n\rangle\backslash\\{1\\}.
\\]
```
# Fibonacci

We implement two algorithms for computing the nth term of the Fibonacci sequence

$$
\begin{equation*}
\forall n \in \mathbb{N}, \ u_n =
\begin{cases}
n & \text{if } n < 2 \\
u_{n-1} + u_{n-2} & \text{if } n \geq 2
\end{cases}
\end{equation*}
$$
- naive_fibonacci &rarr; $\mathcal{O}(2^n)$
- smart_fibonacci &rarr; $\mathcal{O}(n)$

## Usage

- [Install pixi](https://pixi.sh/dev/)
- [Pixi for rust projects](https://pixi.sh/dev/tutorials/rust/)


```
pixi run start
```

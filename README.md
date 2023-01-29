# No Code Machine Learning

Suppose we have this sample network:

<p align="center">
  <img alt="Sample network" src="docs/net.png">
  
  <p align="center">
    <sup>$X$...inputs</sup>
</p>
</p>

<!-- definition of the activation -->
```math
a^{(l)}_j = f(z^{(l)}_j)
```

Where $f$ is the activation function, e.g., sigmoid:

<!-- example of activation with the sigmoid activation function -->
```math
a^{(l)}_j = \sigma(z^{(l)}_j)
```

<!-- definition of z -->
```math
z^{(l)}_j = \sum_{k = 0}^{n_l - 1} (a^{(l - 1)}_k w^{(l)}_{jk}) + b^{(l)}_j
```
<p align="center">
  <sup>$n_l$...number of inputs in the layer $l$</sup>
</p>

<p align="center">
  <sup>$l$...any layer in the network</sup>
</p>

## All formulas for all a and z values

### $L = 3$
<!-- formula for a^{(3)}_0 -->
```math
a^{(3)}_0 = f(z^{(3)}_0)
```

<!-- formula for z^{(3)}_0 -->
```math
z^{(3)}_0 = {
  a^{(2)}_0 w^{(3)}_{00} + 
  a^{(2)}_1 w^{(3)}_{01} + 
  b^{(3)}_0
}
```

<!-- formula for a^{(3)}_1 -->
```math
a^{(3)}_1 = f(z^{(3)}_1)
```

<!-- formula for z^{(3)}_1 -->
```math
z^{(3)}_1 = {
  a^{(2)}_0 w^{(3)}_{10} + 
  a^{(2)}_1 w^{(3)}_{11} + 
  b^{(3)}_1
}
```


### $L = 2$
<!-- formula for a^{(2)}_0 -->
```math
a^{(2)}_0 = f(z^{(2)}_0)
```

<!-- formula for z^{(2)}_0 -->
```math
z^{(2)}_0 = {
  X_0 w^{(2)}_{00} + 
  X_1 w^{(2)}_{01} + 
  b^{(2)}_0
}
```

<!-- formula for a^{(2)}_1 -->
```math
a^{(2)}_1 = f(z^{(2)}_1)
```

<!-- formula for z^{(L)}_1 -->
```math
z^{(2)}_1 = {
  X_0 w^{(2)}_{10} + 
  X_1 w^{(2)}_{11} + 
  b^{(2)}_1
}
```

## Activation functions and their derivatives
 
### Sigmoid

<!-- definition of sigmoid activation function -->
```math
\sigma(z) = \frac{1}{1 + e^{-z}}
```

<!-- derivative of sigmoid -->
```math
\sigma'(z) = \sigma(z)(1 - \sigma(z))
```

### ReLU

<!-- definition of relu activation function -->
```math
ReLU(z) = max(0, z)
```

<!-- derivative of relu -->
```math
ReLU'(z) = \left\{
  \begin{array}{ l l }
    1 \qquad \textrm{if $z > 0$} \\ 0 \qquad \textrm{if $z \leq 0$*}
  \end{array}
\right.
```

<p align="center">
  <sup>*The derivative at zero is not defined. The predefined function in the program returns 0.</sup>
</p>

## Derivatives of cost functions

Suppose a universal cost function, $C$.

<!-- partial derivative of C with respect to w^{(L)}_{jk} -->
```math
\frac{\partial C}{\partial w^{(L)}_{jk}} = {
  \frac{\partial C}{\partial a^{(L)}_j}
  \frac{\partial a^{(L)}_j}{\partial z^{(L)}_j}
  \frac{\partial z^{(L)}_j}{\partial w^{(L)}_{jk}}
}
```

<!-- partial derivative of C with respect to b^{(L)}_j -->
```math
\frac{\partial C}{\partial b^{(L)}_j} = {
  \frac{\partial C}{\partial a^{(L)}_j}
  \frac{\partial a^{(L)}_j}{\partial z^{(L)}_j}
  \frac{\partial z^{(L)}_j}{\partial b^{(L)}_j}
} 
```

<!-- partial derivative of C with respect to a^{(L - 1)}_k -->
```math
\frac{\partial C}{\partial a^{(L - 1)}_k} = {
  \frac{\partial C}{\partial a^{(L)}_j}
  \frac{\partial a^{(L)}_j}{\partial z^{(L)}_j}
  \frac{\partial z^{(L)}_j}{\partial a^{(L - 1)}_k}
}
```

<p align="center">
  <sup>Where $L$ is the last layer</sup>
</p>

### Mean Squared Error (MSE)

<!-- the definition of mse -->
```math
C = MSE = {
  \frac{1}{n_L}
  \sum_{j = 0}^{n_L-1}(a^{(L)}_j - y_j)^2
}
```

<!-- the derivative of mse -->
```math
\frac{\partial C}{\partial a^{(L)}_j} = 2(a^{(L)}_j - y_j)
```

<br>

<!-- partial derivative of C with respect to w^{(L)}_{jk} -->
```math
\frac{\partial C}{\partial w^{(L)}_{jk}} = {
  \frac{\partial C}{\partial a^{(L)}_j}
  \frac{\partial a^{(L)}_j}{\partial z^{(L)}_j}
  \frac{\partial z^{(L)}_j}{\partial w^{(L)}_{jk}}
} = {
  2(a^{(L)}_j - y_j)
  f'(z^{(L)}_j)
  a^{(L - 1)}_k
}
```

<!-- partial derivative of C with respect to b^{(L)}_j -->
```math
\frac{\partial C}{\partial b^{(L)}_j} = {
  \frac{\partial C}{\partial a^{(L)}_j}
  \frac{\partial a^{(L)}_j}{\partial z^{(L)}_j}
  \frac{\partial z^{(L)}_j}{\partial b^{(L)}_j}
} = {
  2(a^{(L)}_j - y_j)
  f'(z^{(L)}_j)
  1
} = {
  2(a^{(L)}_j - y_j)
  f'(z^{(L)}_j)
}
```

<!-- partial derivative of C with respect to a^{(L - 1)}_k -->
```math
\frac{\partial C}{\partial a^{(L - 1)}_k} = {
  \frac{\partial C}{\partial a^{(L)}_j}
  \frac{\partial a^{(L)}_j}{\partial z^{(L)}_j}
  \frac{\partial z^{(L)}_j}{\partial a^{(L - 1)}_k}
} = {
  \sum_{j=0}^{n_L - 1}
  2(a^{(L)}_j - y_j)
  f'(z^{(L)}_j)
  w^{(L)}_{jk}
}
```

For any layer $l$ in the network:

<!-- partial derivative of C with respect to w^{(l)}_{jk} -->
```math
\frac{\partial C}{\partial w^{(l)}_{jk}} = {
  \frac{\partial C}{\partial a^{(l)}_j}
  f'(z^{(l)}_j)
  a^{(l - 1)}_k
}
```

<!-- partial derivative of C with respect to b^{(l)}_j -->
```math
\frac{\partial C}{\partial b^{(l)}_j} = {
  \frac{\partial C}{\partial a^{(l)}_j}
  f'(z^{(l)}_j)
}
```

Where:

<!-- partial derivative of C with respect to a^{(l)}_j if l = L -->
```math
\frac{\partial C}{\partial a^{(l)}_j} = 2(a^{(L)}_j - y_j) \qquad 
\textrm{if $l=L$}
```

<p align="center">
  <sup>$L$...last layer of the network</sup>
</p>

<!-- partial derivative of C with respect to a^{(l)}_j if l != L -->
```math
\frac{\partial C}{\partial a^{(l)}_k} = {
  \sum_{j=0}^{n_{L + 1} - 1}
  \frac{\partial C}{\partial a^{(l + 1)}_j}
  f'(z^{(l + 1)}_j)
  w^{(l + 1)}_{jk}
} \qquad
\textrm{otherwise}
```

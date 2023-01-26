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
a^{(L)}_j = f(z^{(L)}_j)
```

Where $f$ is the activation function, e.g., sigmoid:

<!-- example of activation with the sigmoid activation function -->
```math
a^{(L)}_j = \sigma(z^{(L)}_j)
```

<!-- definition of z -->
```math
z^{(L)}_j = \sum_{k = 0}^{n_L - 1} (a^{(L - 1)}_k w^{(L)}_{jk}) + b^{(L)}_j
```
<p align="center">
  <sup>$n_L$...number of inputs in the layer L</sup>
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

## Derivatives of cost functions

### Mean Squared Error (MSE)

<!-- the definition of mse -->
```math
C = MSE = \sum_{j = 0}^{n_L-1}(a_j^{(L)}-y_j)^2
```

<!-- the derivative of root -->
```math
root = {
  \frac{\partial C}{\partial a^{(L)}}
  \frac{\partial a^{(L)}}{\partial z^{(L)}}
}
```

<p align="center">
  <sup>root is the common part for derivative with respect to weight, bias and activation in the previous layer</sup>
</p>

```math
\frac{\partial C}{\partial w^{(L)}} = {
  root
  \frac{\partial z^{(L)}}{\partial w^{(L)}}
}
```

```math
\frac{\partial C}{\partial b^{(L)}} = {
  root
  \frac{\partial z^{(L)}}{\partial b^{(L)}}
}
```

```math
\frac{\partial C}{\partial a^{(L - 1)}} = {
  root
  \frac{\partial z^{(L)}}{\partial a^{(L - 1)}}
}
```

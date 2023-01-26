# No Code Machine Learning
## Derivatives of cost functions
### Mean Squared Error (MSE)
```math
C = MSE = \sum_{j = 0}^{n_L-1}(a_j^{(L)}-y_j)^2
```

```math
\frac{\partial C}{\partial w^{(L)}} = {
  \frac{\partial C}{\partial a^{(L)}}
  \frac{\partial a^{(L)}}{\partial z^{(L)}}
  \frac{\partial z^{(L)}}{\partial w^{(L)}}
} = {
  a^{(L - 1)}
  \sigma'(z^{(L)})
  2(a^{(L)}-y)
}
```

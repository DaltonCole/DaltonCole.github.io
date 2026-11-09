+++
title = "Activation Functions"
math = true
+++

The [activation function](https://keras.io/api/layers/activations/) determines what is outputted by neurons of this layer. There are two ways to add an activation function to a layer. You can either add it via the `activation` argument on any layer or you can add the activation function as a layer.

- Argument:

```python
model.add(keras.layers.Dense(32, activation='relu'))
```

- As a layer:

```python
model.add(keras.layers.Dense(32))
model.add(keras.layers.Activation('relu'))
```

Built in activation functions:

- relu
  - The ReLU or rectified linear unit activation function: `max(x, 0)`
  - Is the generic activation function.
  - To clip, you can set the max value using the `max_value` argument or the min value using the `threshold` argument.
- sigmoid
  - $\theta (x) = \frac{1}{1 + e^{-x}}$
  - Values are between 0 and 1
  - S-Shaped
  - Great for making the last layer output a probability.
- softmax
  - Converts a real vector to a vector of categorical probabilities.
  - Each output is the probability of that output. All outputs sum to 1.
  - Generally used as the activation function for the *last layer*.
- softplus
  - $softplus(x) = log(e^x + 1)$
- softsign
  - $softsign(x) = \frac{x}{\lvert x \rvert + 1}$
- tanh
  - Hyperbolic tangent. $tanh(x) = \frac{sinh(x)}{cosh(x)} = \frac{e^x - e^{-x}}{e^x + e^{-x}}$
  - Outputs are between (-1, 1)
  - S-Shaped
  - Advantages: •Negative numbers are perserved •Zero inputs are mapped near zero
- selu
  - The SELU or scaled exponential linear unit is related to the ReLU activation function and super related to the Leaky ReLU activation function.
    - `if x ≥ 0: return scale * x`
    - `if x ≤ 0: return scale * alpha * (exp(x) - 1)`
  - Unlike ReLU, SELU allows negative values, so these cells cannot die (all become 0).
  - Compared to leaky ReLU, there is an exponential dip instead of a straight line for negative values.
- elu
  - Exponential Linear Unit. SELU, but without the scale.
- exponential

Advanced Activation Functions:

- LeakyReLU
  - `tf.keras.layers.LeackyReLU(alpha=0.3)`
  - Like exponential linear unit, but negative values are a linear line with a slight slope.

Pros and Cons of most activation function: [HERE](https://mlfromscratch.com/activation-functions-explained/#/).

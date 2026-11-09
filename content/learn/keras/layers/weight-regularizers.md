+++
title = "Weight Regularizers"
+++

[Weight regularizers](https://keras.io/api/layers/regularizers/) penalizes certain aspects of a layer's parameters during optimization (training).

Three common regulaizers exist for most layer types:

- `kernel_regularizer`: Applies regularization function to the weights matrix
- `bias_regularizer`: Applies regularization function to the bias
- `activity_regularizer`: Applies regularization function to the output of the layer

Good stackexchange [post](https://stats.stackexchange.com/questions/383310/what-is-the-difference-between-kernel-bias-and-activity-regulizers-and-when-t) about the three regularizers.

There are three available regularizers:

- `tf.keras.regularizers.l1(l1=0.01)`: `loss = l1 * reduced_sum(abs(x))`
- `tf.keras.regularizers.l2(l1=0.01)`: `loss = l1 * reduced_sum(square(x))`
- `tf.keras.regularizers.l1_l2(l1=0.01, l2=0.02)`

For example:

```python
layer = tf.keras.layers.Dense(
    units=64,
    kernel_regularizer=regularizers.l1_l2(l1=1e-5, l2=1e-4),
    bias_regularizer=regularizers.l2(1e-4),
    activity_regularizer=regularizers.l2(1e-5)
)
```

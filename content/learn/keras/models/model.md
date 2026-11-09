+++
title = "Model"
+++

The `Model` class requires two things, the `inputs` to a model and the `outputs` to a model. There is an optimal `name` parameter

There are two ways to instantiate a `Model` class:

- Functions API
- By subclassing the `Model` class

Function API method:

```python
import tensorflow as tf
inputs = tf.keras.Input(shape=(3,))
x = tf.keras.layers.Dense(4, activation=tf.nn.relu)(inputs)
outputs = tf.keras.layers.Dense(5, activation=tf.nn.softmax)(x)
model = tf.keras.Model(inputs=inputs, outputs=outputs)
```

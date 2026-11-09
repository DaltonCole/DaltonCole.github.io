+++
title = "Sequential"
+++

The [Sequential](https://keras.io/api/models/sequential/) class allows you to add layers sequentially to a model.

```python
model = tf.keras.Sequential()
model.add(tf.keras.Input(shape=(16,)))  # Add input layer that accepts a feature vector of length 16
model.add(tf.keras.layers.Dense(6))  # Adds a layer containing 6 neurons
```

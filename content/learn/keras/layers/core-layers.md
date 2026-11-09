+++
title = "Core Layers"
+++

##### [Input](https://keras.io/api/layers/core_layers/input/)

Used to instantiate a keras tensor

```python
tf.keras.Input(
    shape=None,
    batch_size=None,
    name=None,
    dtype=None,
    sparse=False,
    tensor=None,
    ragged=False,
    **kwargs
)
```

- Shape: Input shape, not including batch size. Should be a tuple of integers.

##### [Dense](https://keras.io/api/layers/core_layers/dense/)

The most common layer type. A layer that is completely connected to the previous layer.

```python
tf.keras.layers.Dense(
    units,
    activation=None,
    use_bias=True,
    kernel_initializer="glorot_uniform",
    bias_initializer="zeros",
    kernel_regularizer=None,
    bias_regularizer=None,
    activity_regularizer=None,
    kernel_constraint=None,
    bias_constraint=None,
    **kwargs
)
```

- Units: The number of neurons

##### [Activation](https://keras.io/api/layers/core_layers/activation/)

Add an activation function to the previous layer.

```python
tf.keras.layers.Activation(activation, **kwargs)
```

##### [Embedding](https://keras.io/api/layers/core_layers/embedding/)

[THIS](https://gdcoder.com/what-is-an-embedding-layer/) site does a great job of explaining what an embedding layer does. "an embedding learns tries to find the optimal mapping of each of the unique words to a vector of real numbers. The size of that vectors is equal to the output_dim". An embedding layer maps a vector that consists of a small sample of the vocabulary to a feature vector.

Must be the first layer of a model.

```python
tf.keras.layers.Embedding(
    input_dim,
    output_dim,
    embeddings_initializer="uniform",
    embeddings_regularizer=None,
    activity_regularizer=None,
    embeddings_constraint=None,
    mask_zero=False,
    input_length=None,
    **kwargs
)
```

- Input Dim: Vocabulary size, number of possible unique words in an input vector.
- Output Dim: Dimension of the dense embedding (size of the feature vector for each unique word)
- Input Length: Use if the input is of a constant length. Required if using `Flatten` followed by `Dense` later on.

##### [Masking](https://keras.io/api/layers/core_layers/masking/)

Used primarily in RNNs. Skips timesteps. Good for skipping padding when using LSTM.

```python
tf.keras.layers.Masking(mask_value=0.0, **kwargs)
```

##### [Lambda](https://keras.io/api/layers/core_layers/lambda/)

```python
tf.keras.layers.Lambda(
    function, output_shape=None, mask=None, arguments=None, **kwargs
)
```

- Function: Lambda function

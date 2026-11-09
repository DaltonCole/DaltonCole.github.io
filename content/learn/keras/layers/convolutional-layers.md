+++
title = "Convolutional Layers"
+++

Convolutional layers are used with convolutional neural networks (CNNs)

##### Conv\*D

There are three different convolutional layer dimensions:

- [Conv1D](https://keras.io/api/layers/convolution_layers/convolution1d/): Ex temporal convolutions
- [Conv2D](https://keras.io/api/layers/convolution_layers/convolution2d/): Ex spatial convolutions over images
- [Conv3D](https://keras.io/api/layers/convolution_layers/convolution3d/): Ex spatial convolutions over volumes

```python
tf.keras.layers.Conv2D(
    filters,
    kernel_size,
    strides=(1, 1),
    padding="valid",
    data_format=None,
    dilation_rate=(1, 1),
    groups=1,
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

- Filters: Number of output filers
- Kernel Size: Height and width of the 2D convolutional window. Tuple of ints
- Strides: Specifies the strides (movement of the window) along the height and width.
- Padding:
  - 'valid': No padding. Output shape will be smaller than input shape
  - 'same': Adds padding. Output shape (height/width, not channels/filters) will be the same as the input
- data_format
  - 'channels_last' (default): Expects `(batch_size, height, width, channels)` as input
  - 'channels_first': Expects `(batch_size, channels, height, width)` as input

Input Shape: `batch_shape + (channels, rows, cols)` if data format is channels first

Output Shape: `batch_shape + (filters, new_rows, new_cols)` if data format is channels first. Rows and columns may change due to padding.

##### SeperableConv\*D

Similar to Conv\*D layers, but channels are kept separate at first and then mixed at the end. The 2D version is similar to an Inception Block.

- [SeparableConv1D](https://keras.io/api/layers/convolution_layers/separable_convolution1d/)
- [SeparableConv2D](https://keras.io/api/layers/convolution_layers/separable_convolution2d/)

##### [DepthwiseConv2D](https://keras.io/api/layers/convolution_layers/depthwise_convolution2d/)

Performs the first half of SeperableConv2D, where channels are kept separate.

##### Conv\*DTranspose (Deconvolution)

"Undoes" a convolutional layer. Is generally used to increase the dimensionality (rows and columns) while decreasing the channel number.

- [Conv2DTranspose](https://keras.io/api/layers/convolution_layers/convolution2d_transpose/)
- [Conv3DTranspose](https://keras.io/api/layers/convolution_layers/convolution3d_transpose/)

```python
tf.keras.layers.Conv2DTranspose(
    filters,
    kernel_size,
    strides=(1, 1),
    padding="valid",
    output_padding=None,
    data_format=None,
    dilation_rate=(1, 1),
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

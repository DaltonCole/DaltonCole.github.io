+++
title = "Pooling Layers"
+++

Pooling layers are used to downsample. They are generally used with convolutional layers to reduce the size of the feature space

##### MaxPooling\*D

Max pooling uses passes the max value over a window to the next layer. There are three different pooling layer dimensions:

- [Conv1D](https://keras.io/api/layers/pooling_layers/max_pooling1d/): Ex: temporal data
- [Conv2D](https://keras.io/api/layers/pooling_layers/max_pooling2d/): Ex spatial data (images)
- [Conv3D](https://keras.io/api/layers/pooling_layers/max_pooling3d/): Ex 3D data (spatial or spatio-temporal)

```python
tf.keras.layers.MaxPooling2D(
    pool_size=(2, 2), 
    strides=None, 
    padding="valid", 
    data_format=None, 
    **kwargs
)
```

- Pool Size: Size of the window
- Strides: How far the window moves after each pooling step (int or tuple of ints)
- Padding:
  - 'valid': No padding. `output_shape = (input_shape - pool_size + 1) / strides)`
  - 'same': Output will have the same height/width dimensions as input. `output_shape = input_shape / strides`
- Data Format: 'channels_last' or 'channels_first'

##### AveragePooling\*D

Average pooling passes the average value over a window to the next layer. There are three different pooling layer dimensions:

- [AveragePooling1D](https://keras.io/api/layers/pooling_layers/average_pooling1d/): Ex: temporal data
- [AveragePooling2D](https://keras.io/api/layers/pooling_layers/average_pooling2d/): Ex: spatial data (images)
- [AveragePooling3D](https://keras.io/api/layers/pooling_layers/average_pooling3d/): Ex: 3D data (spatial or spatio-temporal)

```python
tf.keras.layers.AveragePooling2D(
    pool_size=(2, 2), strides=None, padding="valid", data_format=None, **kwargs
)
```

Args same as MaxPooling

##### Other

There are also [GlobalMaxPooling](https://keras.io/api/layers/pooling_layers/global_max_pooling2d/) and [GlobalAveragePooling](https://keras.io/api/layers/pooling_layers/global_average_pooling2d/) varients that don't use a window, but the entire input.

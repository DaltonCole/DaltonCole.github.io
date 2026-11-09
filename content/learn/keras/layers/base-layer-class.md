+++
title = "Base Layer Class"
+++

All layers inherit from the [base layer](https://keras.io/api/layers/base_layer/) class.

```python
tf.keras.layers.Layer(
    trainable=True, name=None, dtype=None, dynamic=False, **kwargs   
)
```

- Trainable: If false, the layer is *frozen* and will not be trained.
- Name: Name a layer. Can be accessed via `model.layers[0].name`

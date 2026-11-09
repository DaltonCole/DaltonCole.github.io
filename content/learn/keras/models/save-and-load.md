+++
title = "Save And Load"
+++

###### Save tf.keras

[Saving](https://www.tensorflow.org/guide/keras/save_and_serialize) a model whole is super easy.

```python
model.save('my_model')
```

This will create a directory called `my_model` with `assets`, `saved_model.pb`, and `variables` as contents. This only works if using `tf.keras` instead of native keras. See below if using native keras.

To load it again:

```python
model = tf.keras.models.load_model('my_model')
```

###### Save keras

[Saving](https://keras.io/api/models/model_saving_apis/) the model as a single HDF5 is an option, however some items are not saved, such as custom layers and external losses and metrics. These can be quite annoying to add back later if you get a model from someone else.

H5:

```python
model.save('model.h5')
model = tf.keras.models.load_model('model.h5')
```

###### [Other Save/Load Functions](https://keras.io/api/models/model_saving_apis/)

- `model.get_weights()`
- `model.set_weights(weights)`
- `model.save_weights('file_path.h5')`
- `model.load_weights('file_path.h5')`
- `model.to_json()`
- `model = tf.keras.models.model_from_json(config)`
- `new_model = tf.keras.models.clone_model(model)`

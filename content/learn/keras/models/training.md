+++
title = "Training"
+++

Keras [training](https://keras.io/api/models/model_training_apis/) APIs involve compiling, fitting, evaluating, and predicting using a model.

###### Compile

Prepares the model for training (does a lot of hidden stuff).

```python
Model.compile(
    optimizer="rmsprop",
    loss=None,
    metrics=None,
    loss_weights=None,
    weighted_metrics=None,
    run_eagerly=None,
    steps_per_execution=None,
    **kwargs
)
```

- Optimizer: [Adam](https://www.tensorflow.org/api_docs/python/tf/keras/optimizers/Adam) is the most popular optimizer.
- Loss: The neural network will try to minimize this value via the optimization algorithm. There are a large number of [loss functions](https://www.tensorflow.org/api_docs/python/tf/keras/losses). Most notably (note, for keras, you can use a string instead of the tensorflow functions by snake-casing the function as a string):
  - Classification (Categorical) Data:
    - [BinaryCrossentropy](https://www.tensorflow.org/api_docs/python/tf/keras/losses/BinaryCrossentropy): Used when there are only two possible labels (0 and 1)
    - [CategoricalCrossentropy](https://www.tensorflow.org/api_docs/python/tf/keras/losses/CategoricalCrossentropy): Used when there are two+ possible classes. Expects labels to be encoded via one-hot representation.
    - [SparseCategoricalCrossentropy](https://www.tensorflow.org/api_docs/python/tf/keras/losses/SparseCategoricalCrossentropy): Sibling to CategoricalCrossentropy. Expects an integer encoding instead of one-hot. Integers are distinct classes, similarity via closeness is not assumed.
  - Regression (Continuous) Data:
    - [MeanSquaredError](https://www.tensorflow.org/api_docs/python/tf/keras/losses/MeanSquaredError): Loss = square(y_true - y_pred)
- Metrics: List of [metrics](https://keras.io/api/metrics/) to output during training and returned during fitting. `['accuracy']` is the most common metric
- Loss Weights: If a list of losses is given as the lost function, you can specify how heavily waited each loss function is. For example `[10, 1]` would weight the first loss function 10 times heavier than the second loss function.

###### Fit

Used to train a model.

```python
Model.fit(
    x=None,
    y=None,
    batch_size=None,
    epochs=1,
    verbose=1,
    callbacks=None,
    validation_split=0.0,
    validation_data=None,
    shuffle=True,
    class_weight=None,
    sample_weight=None,
    initial_epoch=0,
    steps_per_epoch=None,
    validation_steps=None,
    validation_batch_size=None,
    validation_freq=1,
    max_queue_size=10,
    workers=1,
    use_multiprocessing=False,
)
```

- Verbose: 0 = silent, 1 = progress bar per epoch, 2 = one line output per epoch
- Callbacks: List of [callbacks](https://keras.io/api/callbacks/). My documentation on callbacks can be found [here](/learn/keras/callbacks/).
- Validation Split: Float between 0 and 1. Fraction of training data to use for validation. Uses the ending % BEFORE shuffling.
- Validation Data: Data to use for validation. Data should be in `(x_val, y_val)` format. Do not use with `validation_split`.
- Class Weight: Dictionary mapping class indices (integers) to weight (float). Useful for unbalanced data (where there are more samples of one class than another)
- Sample Weight: Weigh samples differently. 1D numpy array of sample size is expected.
- Validation Frequency: How often, in terms of epochs, to validate the data.
- Generator Specific Arguments:
  - Steps Per Epoch: Number of batches required to declare an epoch. Needed for generators. Same idea for validation_steps.
  - Max Queue Size: Number of samples to queue for a generator. Defaults to 10.
  - Workers: Number of workers used for generators. Defaults to 1.
  - Use Multiprocessing: Use process-based threading for generators.

Returns a `History` object that can be used for plotting.

###### Evaluate

Like fit, but without the training. Used to find the loss and metric values for the model.

```python
Model.evaluate(
    x=None,
    y=None,
    batch_size=None,
    verbose=1,
    sample_weight=None,
    steps=None,
    callbacks=None,
    max_queue_size=10,
    workers=1,
    use_multiprocessing=False,
    return_dict=False,
)
```

- Return Dict: Return the loss and metric results as a dictionary instead of a list. Key is the name of the metric. If False, a list (or single value) is returned.

###### Predict

Used to predict data. A batch is expected.

```python
Model.predict(
    x,
    batch_size=None,
    verbose=0,
    steps=None,
    callbacks=None,
    max_queue_size=10,
    workers=1,
    use_multiprocessing=False,
)
```

Numpy array of predictions is returned

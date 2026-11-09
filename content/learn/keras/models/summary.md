+++
title = "Summary"
+++

`Model.summary()` can be used to summarize your model, but outputting the layers

```
Layer (type)                 Output Shape              Param #
=================================================================
input_1 (InputLayer)         [(None, 3)]               0
_________________________________________________________________
dense (Dense)                (None, 4)                 16
_________________________________________________________________
dense_1 (Dense)              (None, 5)                 25
=================================================================
Total params: 41
Trainable params: 41
Non-trainable params: 0
_________________________________________________________________
```

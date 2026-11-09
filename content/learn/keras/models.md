+++
title = "Models"
+++

A [model](https://keras.io/api/models/model/) in keras is just a group of layers.

A complete example that goes through creating, training, and evaluating a keras model:

```python
from sklearn.datasets import load_iris
from sklearn.preprocessing import LabelBinarizer
from sklearn.utils import shuffle
import keras

# Load dataset
iris = load_iris()
data = iris.data
enc = LabelBinarizer()
target = enc.fit_transform(iris.target)
X, y = shuffle(data, target, random_state=0)

# Make model
inputs = keras.Input(shape=(4,))
x = keras.layers.Dense(5, activation='relu')(inputs)
outputs = keras.layers.Dense(3, activation='softmax')(x)
model = keras.Model(inputs=inputs, outputs=outputs)

# Compile model
model.compile(loss='categorical_crossentropy', optimizer='adam', metrics=['accuracy'])

# Train
model.fit(x=X, y=y, batch_size=8, epochs=150, validation_split=0.3)

# Evaluate
loss, accuracy = model.evaluate(X, y)

# Predict
predictions = model.predict(X)  # softmax gives us a probability of each category
```

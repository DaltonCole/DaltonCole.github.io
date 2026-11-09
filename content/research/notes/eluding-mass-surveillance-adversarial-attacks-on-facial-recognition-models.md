+++
title = "Eluding Mass Surveillance: Adversarial Attacks on Facial Recognition Models"
date = "2018-01-01"
paper_url = "http://cs229.stanford.edu/proj2018/report/250.pdf"
tags = ["Adversarial", "Misclassification", "Perturbation"]
citations = 1
+++

#### Summary

Uses deep neural networks for facial recognition. Trained the DNN with noisy and obscured landmarks data to compare results to. Noisy data consisted of a salt-and-pepper noise; they randomly chose some pixels to be solid red, green, or blue. Obscured facial landmarks consisted of identifying facial landmarks, such as the nose and mouth. Once these landmarks were identified, noise was added via [}Gaussian noise](https://en.wikipedia.org/wiki/Gaussian_noise) style, noise following a Gaussian distribution. The DNN trained with normal data performed well. When noise was added, it performed measurably worse, and when obscured landmarks noise was added, it performed even worse. Confidence in the DNN's choices shrunk with each attack.

#### Notes

- [Code](https://github.com/amilich/face)
- Used a modified model from [hackernoon](https://hackernoon.com/building-a-facial-recognition-pipeline-with-deep-learning-in-tensorflow-66e7645015b8) for facial recognition. Required cropping, rotation, and alignment pre-processing.
- Facial recognition DNN generated a 128-dimensional embedding for each face that was feed into a SVM classifier for face classification.

#### Analysis

- They required a separate DNN to find facial landmarks. A [kaggle](https://www.kaggle.com/madhawav/basic-fully-connected-nn) dataset was used for the facial landmarks while the [LFW](http://vis-www.cs.umass.edu/lfw/) dataset was used for the facial recognition DNN. Using two different datasets may have decreased accuracy.
- Used a pre-trained network, then applied transfer learning for their LFW samples. This might have negatively affected results.

Citation: Milich, Andrew, and Michael Karr. "Eluding Mass Surveillance: Adversarial Attacks on Facial Recognition Models." (2018).

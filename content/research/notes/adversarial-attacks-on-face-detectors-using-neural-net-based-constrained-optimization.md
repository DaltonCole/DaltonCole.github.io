+++
title = "Adversarial Attacks on Face Detectors using Neural Net based Constrained Optimization"
date = "2018-05-31"
paper_url = "https://arxiv.org/pdf/1805.12302.pdf"
tags = ["Adversarial", "Defense", "Perturbation"]
citations = 37
+++

#### Summary

Used a generative neural network and a Faster R-CNN identification network to generate perturbed samples to misclassify facial detection (existence of a face in an image). Briefly tested JPEG compression as a counter measure. Only 0.5% of samples were correctly recognized with no defense. With the JPEG defense, 5.0% of samples were correctly recognized.

#### Notes

- Required a Faster R-CNN to recognize if an image contained a face or not. Depending on how the facial recognition classifier is made, this made the solution somewhere between a black-box and a white-box attack.
- Generative neural network could easily generate faces that do not look realistic. The Faster R-CNN was used to make sure that did not happen, but faces with only a 0.7 certainty were accepted when training and only 0.5 when testing.
- Trained and tested on a small dataset of 600 images
- Used a pre-trained Faster R-CNN network that has been trained on normal samples
- Multiple gradient steps on the same image must be performed. This can be quite expensive.

#### Interesting References

- "Neural networks have been proven to be universal function approximators" - [SITE](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.101.2647&rep=rep1&type=pdf)

Citation: Bose, Avishek Joey, and Parham Aarabi. "Adversarial attacks on face detectors using neural net based constrained optimization." 2018 IEEE 20th International Workshop on Multimedia Signal Processing (MMSP). IEEE, 2018.

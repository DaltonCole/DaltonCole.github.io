+++
title = "MagNet: A Two-Pronged Defense against Adversarial Examples"
date = "2017-09-11"
paper_url = "https://dl.acm.org/doi/10.1145/3133956.3134057"
tags = ["Adversarial", "Detection"]
citations = 463
math = true
+++

#### Summary

They used two extra neural networks. One is a detector, which determines if a sample is adversarial or not. If not adversarial, the sample goes through a reformer to bring the sample closer to the manifold, which increases the likelihood of correct classification. The manifold is the line in the feature space that represents normal examples. This is outlined below. Note, no adversarial examples were used.

#### Notes

- There are three ways to defend against adversarial examples:
  - Adversarial Training - Training with adversarial examples
  - Distinguish between normal and adversarial examples - Required adversarial examples
  - Defensive Distillation
- [Carlini & Wagner](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=7958570) showed that defensive distillation did not significantly increase the robustness of neural networks. Defensive distillation is: "making target classifiers hard to attack by blocking gradient path-way - [Papernot et al.](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=7546524)
- The reformer used was an autoencoder - a neural network trained to attempt to copy its input to its output
- Jensen-Shannon divergence used for the autoencoder (reformer) as a similarity metric
- Distance metrics used: $L^0, L^2, L^\infty$, i.e. the $L^P$ norm
- Tested against 4 attacks: Fast gradient sign method, iterative gradient sign method, deepfool, Carlini attack
- Uses a detecting adversarial examples method that does not require adversarial samples
- Uses "reconstruction error" to estimate how far a test example is from the minfold of normal examples

#### Analysis

- Performed very poorly on whitebox attacks. They changed this to a graybox attack by randomising which set of neural networks to use.
- Good results against Carlini when using a graybox approach. Table 6,7,8 in paper shows the results.

Citation: Meng, Dongyu, and Hao Chen. "Magnet: a two-pronged defense against adversarial examples." Proceedings of the 2017 ACM SIGSAC conference on computer and communications security. 2017.

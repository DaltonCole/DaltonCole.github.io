+++
title = "ADVBOX: A TOOLBOX TO GENERATE ADVERSARIAL EXAMPLES THAT FOOL NEURAL NETWORKS"
date = "2020-08-26"
paper_url = "https://arxiv.org/pdf/2001.05574.pdf"
tags = ["Adversarial", "Framework", "Perturbation"]
citations = 6
+++

#### Summary

Advbox is a toolbox used to generate adversarial examples that fool neural networks. It can also be used as a machine learning benchmarking tool.

The following frameworks are supported: Tensorflow, Keras, PyTorch, PaddlePaddle, Caffe2, and MxNet. A number of attack and defense methods are also supported.

[Code](https://github.com/advboxes/AdvBox)

#### Notes

- Focuses on perturbation attacks
- Supported adversarial attacks (section 4.3):
  - FGSM
  - BIM
  - DeepFool
  - JSMA
  - CW
  - PGD
- Supported defense algorithms (section 4.4.):
  - Feature Squeezing
  - Spatial Smoothing
  - Label Smoothing
  - Gaussian Augmentation
  - Adversarial Training
  - Thermometer Encoding
- Uses [Perceptron](https://github.com/advboxes/perceptron-benchmark) to evaluate tests. Supported attack and evaluation approaches:
  - CW
  - Gaussian Noise
  - Uniform Noise
  - Pepper Noise
  - Gaussian Blurs
  - Brightness
  - Rotations
  - Bad Weather

#### Interesting References

- 

#### Analysis

- 

#### Ideas

- 

Citation: Goodman, Dou, et al. "Advbox: a toolbox to generate adversarial examples that fool neural networks." arXiv preprint arXiv:2001.05574 (2020).

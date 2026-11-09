+++
title = "Xception: Deep Learning with Depthwise Separable Convolutions"
date = "2017-04-04"
paper_url = "https://arxiv.org/pdf/1610.02357.pdf"
tags = ["CNN"]
citations = 3008
+++

#### Summary

Convolutional Neural Network (CNN) architecture that outperforms Inception V3 using fewer parameters. There is a [Keras implementation](https://keras.io/api/applications/xception/)

#### Notes

- Inspired by the Inception architecture. Xception has fewer parameters and generally performs better.
- The Inception hypothesis:
  - "A convolution layer attempts to learn filters in a 3D space, with 2 spatial dimensions (width and height) and a channel dimension; thus a single convolution kernel is tasked with simultaneously mapping cross-channel correlations and spatial correlations."
  - "the typical Inception module first looks at crosschannel correlations via a set of 1x1 convolutions, mapping the input data into 3 or 4 separate spaces that are smaller than the original input space, and then maps all correlations in these smaller 3D spaces, via regular 3x3 or 5x5 convolutions."
  - "the fundamental hypothesis behind Inception is that cross-channel correlations and spatial correlations are sufficiently decoupled that it is preferable not to map them jointly."
  - Figures 1-4 show Inception modules in different formulations
  - Depthwise separable convolutions are depthwise. Inception performs a depthwise convolution (spatial convolution) on each channel followed by a pointwise convolution (projecting the channels output by the depthwise convolution onto a new channel space)
- Xception decouples the cross-channel correlations and spatial correlations from the Inception model.
- Figure 5 shows the Xception architecture (Top-Down view).
- Xception outperforms Inception V3 on image classification tasks, despite not being optimally trained using hyper-parameter tuning.

Citation: Chollet, François. "Xception: Deep learning with depthwise separable convolutions." Proceedings of the IEEE conference on computer vision and pattern recognition. 2017.

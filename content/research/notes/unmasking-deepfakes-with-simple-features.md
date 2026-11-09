+++
title = "Unmasking DeepFakes with simple Features"
date = "2020-03-04"
paper_url = "https://arxiv.org/pdf/1911.00686.pdf"
tags = ["Dataset", "Deepfake", "Detection"]
citations = 9
+++

#### Summary

Discrete Fourier Transformation → Azimuthal Average → Classifier → Deepfake Classification

Used the power spectrum results from a Discrete Fourier Transformer (power and phase is produced, but only power is used). To reduce the dimensionality of the image from a 2D to 1D representation, the Azimuthal Average was used. Finally, a classifier was applied to determine if a sample was a deepfake or not. Using the DeepFakeDetection dataset portion of FaceForensics++, they achieved 90% accuracy per video using a SVM classifier. Depending on the dataset, as little as 20 samples were required to achieve 100% accuracy.

This paper showed that it is possible to achieve good results using little training data.

It also showed that unsupervised methods generally perform worse than supervised methods for deepfake classification.

#### Notes

- Figure 2 summarizes their framework
- Gray-scale images were used
- "The Discrete Fourier Transform (DFT) is a mathematical technique to decompose a discrete signal into sinusoidal components of various frequencies ranging from 0 (i.e., constant frequency, corresponding to the image mean value) up to the maximum representable frequency, given the spatial resolution."
- An Azimuthal Average can be seen as a compression, gathering, and averaging of similar frequency components into a vector of features.
- Three different classifiers were looked at: Logistic Regression, SVM, and K-Means Clustering. Two supervised and one unsupervised method.
- Generated a new deepfake image dataset called ***Faces-HQ***. This dataset is summarized in table 1. It is composed of 40,000 images, half are real, half deepfake.
- Table 2 shows the results using Faces-HQ. SVM and Logistic Regression had 100% accuracy with as little as 20 photos.
- Tables 3, 4, and 5 show that later features are more important than earlier features using this preprocessing method.
- Table 6 shows the results using the CelebA dataset. 100% accuracy using SVM and Logistic Regression using 2000 images.
- Table 7 shows the results using the DeepFakeDetection dataset from FaceForensics++. 85% accuracy was achieved when 2000 samples were used, and 66% when 20 samples were used. SVM outperformed Logistic Regression.
- Table 8 shows the results using videos from the DeepFakeDetection dataset instead of single frames. 90% accuracy was achieved when using a SVM

#### Interesting References

- Image Forensics
  - Local noise estimination
  - Pattern analysis
  - Illumination modeling
  - Steganalysis feature classification
  - CNN

#### Analysis

- I'm not sure about DFT and the Azimuthal Average, but the classifiers used would be much quicker than a CNN

Citation: Durall, Ricard, et al. "Unmasking deepfakes with simple features." arXiv preprint arXiv:1911.00686 (2019).

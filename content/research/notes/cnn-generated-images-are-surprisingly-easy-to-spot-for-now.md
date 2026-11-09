+++
title = "CNN-generated images are surprisingly easy to spot... for now"
date = "2020-04-04"
paper_url = "https://arxiv.org/pdf/1912.11035.pdf"
tags = ["CNN", "Dataset", "Detection"]
citations = 24
+++

#### Summary

Used a binary classifier trained on a single CNN based generator (ProGAN) to determine if an image was real or synthetic. They then tested the classifier on different GANs to see how well it generalized. It generalized really well on StyleGAN, BigGAN, CycleGAN, StarGAN, GauGAN, CRN, IMLE, and SITD. Had trouble on SAN and DeepFake. They also tested to see how different post-processing strategies worked during training, specifically: blur only, JPEG compression only, or both. In addition, the number of classes used for training (ex., cat, dog, table, etc.) was explored. Figure 2 tabularizes these results and Figures 2-4 visualize them. [Github Repo.](https://github.com/PeterWang512/CNNDetection)

#### Notes

- Many archtectures use similar post-processing techniques, which is critical for generalization.
- Diversity of training images matter. They found that having training images from 16+ classes worked best, with diminishing results after 16 classes.
- All GANs tested have an upsampling convolutional structure, which is the most common design for generative CNNs
- Dataset: LSUN object vategories. 36K training images, 200 validation images for each of 20 categories ⇒ 720K images for training, 4K for validation derived from ProGAN.
- ResNet-50 was used as the classifier, which was pre-trained with ImageNet before this training
- Four different augmentation strategies were explored. (1) No augmentation, (2) Gaussian Blur with 50% chance, (3) JPEG compression with 50% chance, (4a) Blur+JPEG at 50% chance, (4b) Blur+JPEG at 10% chance
- **Findings**
  - Augmentation helped, except in the DeepFake and SAN case. SAN uses super-resolution images, thus blurring and compression reduced the artifacts generated via super-resolution. They don't know why DeepFake augmentation performed poorly.
- **Future**
  - In-the-wild testing is needed
  - This work does not generalize well to shallow methods, such as photoshop ⇒ shallow methods are fundamentally different and should not be neglected
  - None of the GAN-based architectures were able to beat their discriminator. When they do, scary stuff may happen.

#### Interesting References

- [Checkerboard](https://distill.pub/2016/deconv-checkerboard/?source=post_page---------------------------) artifact produced by deconvolutional layers

#### Ideas

- How well would this method work on perturbed samples, that are meant to be misclassified by another detector? Does perturbation work across multiple GANs? Is this too similar to a universal filter?

Citation: Wang, Sheng-Yu, et al. "CNN-generated images are surprisingly easy to spot... for now." Proceedings of the IEEE Conference on Computer Vision and Pattern Recognition. Vol. 7. 2020.

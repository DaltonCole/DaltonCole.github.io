+++
title = "Unravelling Robustness of Deep Learning Based Face Recognition against Adversarial Attacks"
date = "2018-02-22"
paper_url = "https://arxiv.org/pdf/1803.00401.pdf"
tags = ["Adversarial", "Detection", "Framework"]
citations = 67
+++

#### Summary

Three main contributions: framework to evaluate face recognition systems, scheme to detect adversarial attacks on a system, method to mitigate adversarial attacks.

#### Notes

- Applied three different distortion methods to images
  - A grid based distortion method picked two points in the image and drew a line between them. The line was then set to a 0 greyscale value.
  - Another distortion method was flipping the most significant bit. Three sets of pixels were chosen, possibly overlapping. The most significant bit in each of these sets were flipped.
  - The third distortion was a black box over certain facial landmarks, such as the forehead and brow, eye region, and beard region.
  - Out of all methods, black box around the eye region had the greatest effect.
- To detect adversarial attacks, internal layers were studied. Layers sensitive to noisy data were found using network visualization analysis. The mean of these layers given normal images was calculated. The Canberra distance between images and the mean was feed into a SVM for two-class classification (normal vs adversarial).
- To mitigate distortion, layers that contain the most filters that are adversely affected by distorted data were found using equation 5 in the paper. Selective dropout was used by setting the weights pertaining to these features (nodes) to 0. The idea behind this was to remove the most problematic filters from the pipeline.

#### Analysis

- Simplistic, yet effective, distortion methods. Even though these methods are effective, they are visually altered enough to be easily detectable by a human.
- Studying internal nodes as used in the paper was a clever method

Citation: Goswami, Gaurav, et al. "Unravelling robustness of deep learning based face recognition against adversarial attacks." arXiv preprint arXiv:1803.00401 (2018).

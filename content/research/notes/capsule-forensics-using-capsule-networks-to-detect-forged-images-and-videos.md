+++
title = "CAPSULE-FORENSICS: USING CAPSULE NETWORKS TO DETECT FORGED IMAGES AND VIDEOS"
date = "2018-10-26"
paper_url = "https://arxiv.org/pdf/1810.11215.pdf"
tags = ["CNN", "Deepfake", "Detection"]
citations = 44
+++

#### Summary

Used a VGG-19 network followed by a capsule-forensics CNN. Produced results comparable or better than the state-of-the-art on the FaceForensics dataset.

#### Notes

- Used a pre-trained VGG-19 network and a CNN
- CNN was a capsule-forensics network (shown in Figure 2)
- Had statistical-pooling layers, which is important for forgery detection
- Added Gaussian random noise to reduce over-fitting
- Appied a squash function to make the network more stable
- Used the FaceForensics dataset
- Table 4 shows their results compared to other methods
  - 99.37% Accuracy on Raw Image
  - 96.50% Accuracy on High Quality Image
  - 81.00% Accuracy on Low Quality Image
- Tested their network on both a frame level and video level. When video level was used, they averaged the outputted probabilities

Citation: Nguyen, Huy H., Junichi Yamagishi, and Isao Echizen. "Capsule-forensics: Using capsule networks to detect forged images and videos." ICASSP 2019-2019 IEEE International Conference on Acoustics, Speech and Signal Processing (ICASSP). IEEE, 2019.

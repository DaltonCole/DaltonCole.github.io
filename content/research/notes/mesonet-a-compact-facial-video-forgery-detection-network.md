+++
title = "MesoNet: a Compact Facial Video Forgery Detection Network"
date = "2018-09-04"
paper_url = "https://arxiv.org/pdf/1809.00888.pdf"
tags = ["Deepfake", "Detection"]
citations = 146
+++

#### Summary

Detect deepfakes via "mesoscopic" image properties, i.e. not small noise nor entire faces at a time (same thing as XceptionNet basically). Proposed networks detect both deepfakes and Face2Face. Two networks are proposed, Meso-4 and MesoInception-4 (has Inception modules). Had 0.917 classification score for single frame deepfake detection and 0.984 for video with some compression. No idea what classification score metric they used (accuracy?). [Github](https://github.com/DariusAf/MesoNet)

#### Notes

- Section 1.1 gives a really good description on how deepfakes are trained (using auto-encoders)
- Deepfake criticisms:
  - "some frames can end up with no facial reenactment or with a large blurred area or a doubled facial contour."
  - "autoencoders tend to poorly reconstruct fine details because of the compression of the input data on a limited encoding space, the result thus often appears a bit blurry."
- Both of the proposed networks performs similarly on deepfakes and Face2Face
- Why mesoscopic: "microscopic analyses based on image noise cannot be applied in a compressed video context where the image noise is strongly degraded. Similarly, at a higher semantic level, human eye struggles to distinguish forged images, especially when the image depicts a human face. That is why we propose to adopt an intermediate approach using a deep neural network with a small number of layers."
- Figure 4 highlights the Meso-4 architecture
- Figure 5 highlights the MesoInception-4 architecture. The first two convolutional layers of Meso-4 are replaced with inception modules/
- Table 2 highlights the datasets used. The deepfake dataset was compressed with the H.264 codec with varying compression levels.
- Faces were extracted using the Viola-Jones detector. Alignment was done via a neural network trained for facial landmark detection.
- They saw a notable deterioration of classification scores when strong video compression was used.
- Performed worse than XceptionNet.
- Found intra-frame aggregation had negative effects on score (intra-frame being frames are not temporally compressed)

#### Analysis

- WHAT CLASSIFCATION METRIC ARE YOU USING?!?
- XceptionNet still #1

Citation: Afchar, Darius, et al. "Mesonet: a compact facial video forgery detection network." 2018 IEEE International Workshop on Information Forensics and Security (WIFS). IEEE, 2018.

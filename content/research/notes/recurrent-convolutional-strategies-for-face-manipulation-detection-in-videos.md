+++
title = "Recurrent Convolutional Strategies for Face Manipulation Detection in Videos"
date = "2019-05-16"
paper_url = "https://arxiv.org/pdf/1905.00582.pdf"
tags = ["Deepfake", "Detection", "RNN"]
citations = 36
+++

#### Summary

Used a RNN for end-to-end deepfake detection. Had 96.9% accuracy on FaceForensics++'s C40 compressed Deepfake dataset.'

#### Notes

- Pipeline: (1) detect, crop, and align faces on a sequence of frames, (2) Use RNN for detection
- Used two different techniques for face alignment: Landmark-based alignment and Spatial transformer network. Landmark-based uses a simple similarity transformation using 7 points of the face (eye points, tip of nose, corners of mouth). The spacial transformer network (STN) performs spatial alignment of data with learnable affine transformation parameters.
- Instead of averaging the probability of the video being malicious across frames, their method directly outputs the results from the RNN. This enables end-to-end training.
- Tested on FaceForensics++'s C40 compression dataset.
- Conclusions:
  - DenseNet outperforms ResNet
  - Sequence of images outperforms a single frame
  - Bidirectional recurrence is superior to uni-directional recurrence
  - Landmark-based alignment outperformed the spacial transformer network
  - Had 96.9% accuracy on FaceForensics++'s C40 compressed Deepfake dataset'

#### Interesting References

- StarGAN alters the background as well, which can lead to artifacts in the background.

#### Analysis

- Although they used the hardest compression scheme for FaceForensics, DeepFakeDetection in FaceForensics is the new hardest dataset to beat in FaceForensics++.

Citation: Sabir, Ekraam, et al. "Recurrent convolutional strategies for face manipulation detection in videos." Interfaces (GUI) 3.1 (2019).

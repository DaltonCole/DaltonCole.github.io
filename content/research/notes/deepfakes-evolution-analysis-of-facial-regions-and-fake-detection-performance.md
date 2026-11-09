+++
title = "DeepFakes Evolution: Analysis of Facial Regions and Fake Detection Performance"
date = "2020-07-02"
paper_url = "https://arxiv.org/pdf/2004.07532"
tags = ["Deepfake", "Detection"]
citations = 1
+++

#### Summary

Looks into detecting deepfakes using Xception and Capsule Network. The first and second generation of datasets are compared with these networks. The effects of using different facial regions are explored, i.e. entire face, eyes, mouth, eyes, and the rest of the face.

The second generation is much harder to detect than the first generation. In general, the eyes (or a single eye) is the most telling feature.

#### Notes

- There are two generations of deepfake datasets. UADFV and FaceForensics++ are in the first generation. Celeb-DF and DFDC Preview are in the second generation. Table 1 summarizes these two datasets.
- The first generation is essentially a solved problem.
- For facial region segmentation, they use the 68 landmarks extracted by OpenFace2. OpenFace2 is robust against: pose variation, distance from the camera, and light conditions. Even works on challenging datasets, such as the DFDC dataset.
- [Xception](/research/notes/xception-deep-learning-with-depthwise-separable-convolutions/) and [Capsule Network](/research/notes/capsule-forensics-using-capsule-networks-to-detect-forged-images-and-videos/)
- A specific fake detector is trained per database and facial region
- Different people are used for training and testing, therefor, the models have to learn the difference between deepfakes and pristine examples, not the people. The same identities were used for each split on all models
- Had roughly ad 80-20 testing-evaluation dataset split
- **Results**

  - Table 3 explores the results.
  - FaceForensics++ proved more challenging than the UADFV dataset, although, both had near 100% accuracy on both models.
  - In general, when the entire face was used provided the best results.
  - Out of all the face regions measured, *eyes* was the best determiner for deepfakes. The only exception was with FaceForensics++, where the mouth was the best determiner. FaceForensics++ has a lack of detail in the teeth and had inconsistencies in lips.
  - Both detectors performed much worse on the second generation datasets compared to the first generation.
  - Figure 3 shows a heat map using Grad-CAM representing the facial features most useful for each fake detector.
  - When only eyes are used, one eye is generally favored over the other.

  | Xception | Face | Eyes |
  | --- | --- | --- |
  | UADFV | 100 | 99.70 |
  | FaceForensics++ | 99.40 | 92.70 |
  | Celeb-DF | 83.60 | 77.30 |
  | DFDC Preview | 91.17 | 83.90 |
  | Capsule Network |  |  |
  | UADFV | 99.90 | 100 |
  | FaceForensics++ | 99.52 | 95.32 |
  | Celeb-DF | 82.46 | 76.64 |
  | DFDC Preview | 87.45 | 83.12 |

Citation: Tolosana, Ruben, et al. "DeepFakes Evolution: Analysis of Facial Regions and Fake Detection Performance." arXiv preprint arXiv:2004.07532 (2020).

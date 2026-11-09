+++
title = "DeepFakes and Beyond: A Survey of Face Manipulation and Fake Detection"
date = "2020-06-18"
paper_url = "https://arxiv.org/pdf/2001.00179.pdf"
tags = ["Deepfake", "Detection", "Survey"]
citations = 25
+++

#### Summary

Survey of various face manipulation methods including deepfakes.

#### Notes

- Paper covers four areas: entire face synthesis (completely generated faces), identity swap (deepfakes), attribute manipulation (change something, ex, age), and expression swap (source-target expression swap).
- **Entire Face Synthesis**
  - Each fake image contains a specific GAN fingerprint, just like natural images can be identified by a device-based fingerprint (PRNU). The fingerprint spans different architectures and different instances. \[ [1](https://ieeexplore.ieee.org/abstract/document/8695364?casa_token=fwg7Z4iUQscAAAAA:wSowF8gOc5Ih5FZIU_GgAAiWlY2dOxvLUjlQB3j8TgQVm97g2eWl6MwSrmDSudSQq98rAD-QXthj), [2](https://openaccess.thecvf.com/content_CVPRW_2019/papers/Media%20Forensics/Albright_Source_Generator_Attribution_via_Inversion_CVPRW_2019_paper.pdf), [3](https://openaccess.thecvf.com/content_CVPRW_2020/papers/w39/Jain_Detecting_GANs_and_Retouching_Based_Digital_Alterations_via_DAD-HCNN_CVPRW_2020_paper.pdf)\]
  - [Neves et al.](https://ieeexplore.ieee.org/abstract/document/9133490?casa_token=zsTtG1lHtpUAAAAA:Q4-R1FhoyHZx1n3AcYK2OUR6O1YzfDw3o6A45IGT_7cqtGkiuTjjqO2403IBYDCmIsaN9JoSojjT) presented the iFakeFaceDB database, consisting of fake images as well as images that had their GAN fingerprints removed via GANprintR (GAN fingerprint Removal). Figure 2 shows an example.
  - Table 2 summarizes the detection approaches for face synthesis
- **Identity Swap** (Deepfakes)
  - Datasets
    - Table 3 summarizes identity swap datasets that are public available.
    - The [FaceForensics++](/research/notes/faceforensics-learning-to-detect-manipulated-facial-images/) is considered a first generation dataset (because it looks fake to be honest).
    - Google added [DeepFakeDetection](https://ai.googleblog.com/2019/09/contributing-data-to-deepfake-detection.html) to the FaceForensics++ database. They look a lot better, however, Google did not release the corresponding models.
  - Detection Methods
    - Table 4 highlights the results. Deep learning features, Image + Temporal features, and Facial Regions features performed the best.
    - Inconsistencies between lip movements and audio speech
    - Eye color, missing reflections, and missing details in the eye and teeth areas
    - Head poses
    - Facial expressions + head movements
    - Eye blinking
    - Artifact detection
    - Mesoscopic properties of the image
    - Classifier + steganalysis features with triplet loss over SVM classification
    - XceptionNet
    - Capsule networks
    - Spatial and motion information based on 3DCNN
    - Temporal-aware pipeline consisting of a CNN and a RNN
    - Temporal discrepancies across frames
  - Detection Methods Summary
    - All show poor generalisation results to unseen databases
    - Poor results on the second generation deepFake databases

#### Interesting References

- From [Faceforensics++](/research/notes/faceforensics-learning-to-detect-manipulated-facial-images/): compression and resizing operations are generally automatically used on social networks.

Citation: Tolosana, Ruben, et al. "Deepfakes and beyond: A survey of face manipulation and fake detection." arXiv preprint arXiv:2001.00179 (2020).

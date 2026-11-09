+++
title = "Exposing Deep Fakes Using Inconsistent Head Poses"
date = "2018-11-13"
paper_url = "https://ieeexplore.ieee.org/abstract/document/8683164?casa_token=FR6loQSUnb4AAAAA:SA8v4vIFiakHcXnfQZYDTbJ625bt6_FRpbpfWcF_QSrYqB01_P4bTI2QjAU64P96yC3HJ0FaBqzL"
tags = ["Deepfake", "Detection", "SVM"]
citations = 76
+++

#### Summary

#### Analysis

- Used area under ROC (AUROC) as the performance evaluation metric. Generally not used.
- Used a very small dataset. Used the UADFV deepfake dataset consisting of 49 real and 49 deepfake videos (corresponding) with an average length of 11.14 seconds. Second dataset was the DARPA MediFor GAN Image/Vieo Challenge dataset, consisting of 241 real images and 252 deepfake images
- Blurry photos decreased the AUROC value (attack vector?)

#### Notes

- Used the difference between the head position and facial feature position to determine if a video was a deepfake or not.
- Figure 1 gives a good overview of their reasoning. When a deepfake is made, the fake face is imposed on the center of the target, without regards to the face boundary. Due to this, the direction the eyes/nose/mouth face may be different than the direction the overall face faces.
- Used the cosine distance between unit vectors to measure difference in head position and face position.
- Used a SVM for classification of the normalized cosine distance features vectors
- Table 1 and Figure 4 shows the results. Per frame classification is meh (max 0.890). Per video is pretty good (max 0.974).

#### Interesting References

- Previous work focused on artifacts or inconsistencies in the video, ex, a lack of realistic eye blinking or mismatched color profiles.

Citation: Yang, Xin, Yuezun Li, and Siwei Lyu. "Exposing deep fakes using inconsistent head poses." ICASSP 2019-2019 IEEE International Conference on Acoustics, Speech and Signal Processing (ICASSP). IEEE, 2019.

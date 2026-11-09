+++
title = "Detection of Deepfake Video Manipulation"
date = "2018-08-01"
paper_url = "https://www.researchgate.net/publication/329814168_Detection_of_Deepfake_Video_Manipulation"
tags = ["Deepfake", "Detection"]
citations = 12
+++

#### Summary

Used photo response non uniformity (PRNU) analysis to determine if a video was a deepfake or not. They found that deepfakes tended to have a lower mean normalized correlation score compared to benign samples. Both deepfakes and benign samples tended to have similar variances in normalized cross correlation scores.

#### Analysis

- Found that benign videos tended to have a higher mean normalized cross correlation score for PRNU analysis compared to deeofakes. There sample size is very small however.

#### Notes

- Used photo response non uniformity (PRNU) analysis to determine if a video was a deepfake or not. PRNU is the unique noise patterns created by small factory defects in light sensitive sensors in digital cameras.
- Dataset consisted of only 10 authentic 20-40 second videos and 16 deepfakes.
- Figure 1 gives and overview of their detection framework. The videos are separated into frames. The faces are then cropped out of them. The faces are split into 8 different groups. The average PRNU pattern fro each group is created via the second order (FSTV) method. The variations in correlation scores and the average correlation score for each video is then calculated.
- It is found that the variance between benign videos and deepfakes are not telling. However, benign videos tend to have a higher mean normalized cross correlation score per video compared to deepfakes.

#### Interesting References

- [Gyfcat](https://www.wired.com/story/gfycat-artificial-intelligence-deepfakes/) (GIF website) spots inconsistencies in the rendering of the facial area of a video. If flagged, the body and background is compared to previously uploaded GIFs. If a video is found and does not match the uploaded video, it is flagged as being manipulated with.

Citation: Koopman, Marissa, Andrea Macarulla Rodriguez, and Zeno Geradts. "Detection of deepfake video manipulation." Conference: IMVIP. 2018.

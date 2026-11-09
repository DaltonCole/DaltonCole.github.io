+++
title = "Face2Face: Real-time Face Capture and Reenactment of RGB Videos"
date = "2016-09-01"
paper_url = "https://openaccess.thecvf.com/content_cvpr_2016/papers/Thies_Face2Face_Real-Time_Face_CVPR_2016_paper.pdf"
tags = ["Facial-Reenactment"]
citations = 744
+++

#### Summary

Created a facial reenactment framework which can be ran in real time on consumer grade hardware. They focused on mouth retrieval.

#### Notes

- Works on RGB videos. Does not require a 3D mask.
- Can be performed in real-time
- Focused on a realistic mouth interior. Does not rely on coping the mouth from the target nor a generic teeth proxy as per other papers.
- Used a multi-linear PCA model as the feature vector. The first two dimensions were facial identity based: geometric shape and skin reflection. The third dimension controlled facial expression.
- Used a facial landmark tracking algorithm for feature alignment
- Mouth Retrieval
  - Similarity Metric: rotation, expression parameters, landmarks, and local binary pattern
  - Frame-to-Cluster Matching: Used k-means clustering with 10 clusters. For each frame, choose a cluster who's cluster representative (min distance to all points within the cluster) was most similar to the target
  - Kept track of a temporal aspect by using the current frame and the previous frame. Used this temporal data for blending.
  - Used alpha blending between the original video frame, the projected mouth frame that has been illuminated-corrected, and the rendered face model

Citation: Thies, Justus, et al. "Face2face: Real-time face capture and reenactment of rgb videos." Proceedings of the IEEE conference on computer vision and pattern recognition. 2016.

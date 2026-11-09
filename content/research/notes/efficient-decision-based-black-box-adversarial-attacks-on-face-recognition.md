+++
title = "Efficient Decision-based Black-box Adversarial Attacks on Face Recognition"
date = "2017-06-01"
paper_url = "https://openaccess.thecvf.com/content_CVPR_2019/papers/Dong_Efficient_Decision-Based_Black-Box_Adversarial_Attacks_on_Face_Recognition_CVPR_2019_paper.pdf"
tags = ["Black-Box", "Misclassification", "Perturbation"]
citations = 44
+++

#### Summary

Used an evolutionary attack algorithm to generate images that either dodge face verification or impersonate someone else with minimal distortion and a fully black-box approach. Resulting images have much less distortion than other leading black-box approaches. The EA works by searching a sub-dimensional space m. Within those subspace, k coordinates are selected randomly for search. Section 3.2 in the paper examines the EA.

#### Notes

- Tested on SphereFace, CosFace, and ArcFace facial recognition models.
- Tested on the LFW and MegaFace datasets
- Decision-based black-box attack
- Uses a variant of the covariance matrix adaptation evolution strategy (CMA-ES) called (1+1)-CMA-ES

#### Interesting References

- CNNs are generally robust to random noises [LINK](https://arxiv.org/pdf/1312.6199.pdf)

#### Analysis

- Required thousands of queries to the classifier to generate an image with little distortion. This could be easily detected

Citation: Dong, Yinpeng, et al. "Efficient decision-based black-box adversarial attacks on face recognition." Proceedings of the IEEE Conference on Computer Vision and Pattern Recognition. 2019.

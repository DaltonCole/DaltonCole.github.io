+++
title = "Face X-ray for More General Face Forgery Detection"
date = "2020-04-19"
paper_url = "https://openaccess.thecvf.com/content_CVPR_2020/papers/Li_Face_X-Ray_for_More_General_Face_Forgery_Detection_CVPR_2020_paper.pdf"
tags = ["Deepfake", "Detection"]
citations = 19
math = true
+++

#### Summary

Used the features generated via blending that most face replacement methods use (ex deepfakes). A boundary box was drawn where the face and the background meet. From this boundary box, the probability of synthetic image was calculated. They did not use fake images to train their model, but real images that underwent a face replacement process. They compared their model against Xception and found that Face X-ray was able to work much better than Xception on datasets the model was not trained on.

#### Notes

- "when an image is formed by blending two images, there exists intrinsic image discrepancies across the blending boundary"
- Manipulated image using mask M, image foreground and image background: $I_M = M \odot I_F + (1 - M) \odot I_B$
- Blending boundary: $B_{i, j} = 4 \cdot M_{i, j} \cdot (1 - M_{i, j})$
  - Max value would be .25, so multiplied by 4 so the bounds are \[0, 1\]
  - $M_{i, j}$ is the value of the mask at point (i, j)
  - The goal of B is to find where the foreground and the background image meet
- They only used a real dataset to train. To generate negative samples, they randomly selected a background image and randomly selected a forground image from the closest 100 neighbours of the background. To blend, they use a random shape deformation followed by a Gaussian blur
- Used a two-phase neural network. The first network was HRNet CNN, followed by a custom network. The first network is used to calculate face X-ray blending boundary. The second network calculates the probability of real or blended.
- Table 1 highlights the results. Their model was able to achieve in general >90.00 AUC on unseen datasets, and generally outperformed XceptionNet.

#### Analysis

- Raw image quality was used to achieve high results. The AUC from HQ and LQ from Faceforensics++ was 87.35% and 61.6% respectively.
- Their method used was trained on alpha blending. Their AUC dropped by 5% when Possion blending was used for validation, showing that this defence is resilient to different blending techniques.
- Blending must be done on the fake image in order to be detected, fully synthetic images (ex GANs) would be unnoticed
- Adversarial samples can be crafted specifically to attack this detector

Citation: Li, Lingzhi, et al. "Face x-ray for more general face forgery detection." Proceedings of the IEEE/CVF Conference on Computer Vision and Pattern Recognition. 2020.

+++
title = "Robust Physical-World Attacks on Deep Learning Visual Classification"
date = "2018-04-10"
paper_url = "https://openaccess.thecvf.com/content_cvpr_2018/html/Eykholt_Robust_Physical-World_Attacks_CVPR_2018_paper"
tags = ["Adversarial", "Misclassification", "Physical"]
citations = 503
+++

#### Summary

Added black and white rectangles to road signs (mainly stop signs) to miss-classify a classifier (not discriminator). Used a white-box approach to figure out the best locations to add the rectangles. Goal was to be discrete and to work from multiple angles.

#### Analysis

- They were able to fool the classifier most of the time (84 to 100% of the time), however, the confidence of the classifier was generally low. See tables 2, 4, and 5 in the paper.

#### Notes

- Took that that cameras must be able to perceive the physical perturbations of the attack. Most digital perturbations would not be picked up (such as the panda example). Colors must be reproducible in the real world.
- Hypothesized that objects have strong and weak physical features from a classification perspective, thus there are stronger and weaker areas on the image. The observed that where they placed the rectangles mattered. To discover mask positions they:
  - Compute perturbations using the *L<sub>1</sub>* regularization and with a mask that occupies the entire surface area of the sign. (BIG mask)
  - Recompute perturbations using *L<sub>2</sub>* with a mask positioned on the vulnerable regions identified in step 1.
- Trained using two different traffic sign datasets: LISA and GTSRB
- The distance and angle did not have a major effect on the accuracy of their attack

Citation: Eykholt, Kevin, et al. "Robust physical-world attacks on deep learning visual classification." Proceedings of the IEEE Conference on Computer Vision and Pattern Recognition. 2018.

+++
title = "Adversarial Perturbations Fool Deepfake Detectors"
date = "2020-05-15"
paper_url = "https://arxiv.org/pdf/2003.10596.pdf"
tags = ["Deepfake", "Detection", "Perturbation"]
citations = 2
+++

#### Summary

Used two different attack that use perturbation (FGSM and CW-L<sub>2</sub>). Attack did well on the VGG and ResNet models. Tried two different defenses. The Lipschitz Regularization defense improved the results somewhat. The Deep Image Prior stopped the attack really well, but was INCREDIBLY slow (30 minutes per image).

#### Analysis

- Tiny dataset for the effective attack
- DIP removed perturbations before feeding it to the classifier. NOTE: won't work with our attack. :)

#### Notes

- Used two different attack methods: [Fast Gradient Sign Method (FGSM)](https://arxiv.org/pdf/1412.6572.pdf) and [Wagner L<sub>2</sub> Norm Attack (CW-L<sub>2</sub>)](https://ieeexplore.ieee.org/abstract/document/7958570/?casa_token=kSnaqvpi1W0AAAAA:ZYyealJFriHUQbzZ7Qe2_9-6XihS0Hd9396L61OPr_RhHm5KSUP8Zv7yj2jUDsLxfUhIMBoIoxys)
- FGSM
  - Popular, efficient, and fast attack
- CW-L<sub>2</sub>
  - Slow but strong attack
- Preformed white and black box attacks. Black-box attack used transfer learning from a ResNet model. Figure 3 shows the attack results. The CS-L<sub>2</sub> attack was more effective but slow.
- Defenses
  - Looked at two different defenses for the perturbation attacks.
  - [Lipschitz Regularization](https://www.nature.com/articles/s42256-019-0104-6)
    - Constrains the gradient of the detector with respect to the input data. It desensitizes the loss from small perturbations.
    - Figure 4 shows the results. The defensed detector generally out performed the un-defended detector, by an okay margin
  - [Deep Image Prior (DIP)](https://openaccess.thecvf.com/content_cvpr_2018/papers/Ulyanov_Deep_Image_Prior_CVPR_2018_paper.pdf)
    - Was originally used for image restoration, such as image denoising, inpainting, and super resolution
    - The goal is to recover **x** given **x**<sub>c</sub>. Equation 9 gives the general equation
    - If the network is optimized for too long, it learns to generate the input image corruptions (bad).
    - This method is SLOW (takes roughly 30 minutes per image), so only 100 images were tested.
    - Table 4 shows the results. "The DIP defense shows more promising results. It achieved a recall of 97.8% for perturbed and unperturbed fake images using a classification threshold of 0.25... the DIP defense retained at least 90.0% of the classifier's performance on real images using the same threshold value." → Decent but SLOW defense

#### Interesting References

- "...machine learning solutions such as deepfake detectors often use existing publicly known and accessible architectures for transfer learning purposes" [LINK](https://arxiv.org/pdf/1909.11573.pdf)
- "...adversarial examples created using a neural network also work on support vector machines and decision tree classifiers" [LINK](https://arxiv.org/pdf/1605.07277.pdf)

Citation: Gandhi, Apurva, and Shomik Jain. "Adversarial perturbations fool deepfake detectors." arXiv preprint arXiv:2003.10596 (2020).

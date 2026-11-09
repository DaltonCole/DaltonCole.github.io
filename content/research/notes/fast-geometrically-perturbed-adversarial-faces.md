+++
title = "Fast Geometrically-Perturbed Adversarial Faces"
date = "2018-09-28"
paper_url = "https://arxiv.org/pdf/1809.08999.pdf"
tags = ["Misclassification", "Perturbation"]
citations = 21
math = true
+++

#### Summary

They created a fast landmark manipulation method for generating adversarial faces. Altered up to five features: jaw, right eye and eyebrow, left eye and eyebrow, nose, and mouth. Distance between features and scale of features were altered. Confirmed that the geometry of the face contains highly discriminate information for face recognition

#### Notes

- [CODE](https://github.com/alldbi/FLM)
- Showed that there is a linear trend between face recognition and landmark locations of the input face image. Figure 6 in paper shows this relationship. Section 4.2 in paper describes what each var represented.
- States that "$l_p-norm$ is not a perfect similarity measure and does not guarantee that the adversarial samples lie on the same manifold as the natural samples".

#### Interesting References

- DNNs are vulnerable to small perturbations in the input domain which can result in drastic change of predictions in the output domain \[[1](https://arxiv.org/abs/1312.6199), [2](https://arxiv.org/abs/1412.6572)\]
- Adversarial examples generated using intensity-based attacks often have high-frequency components that can be used as a measure to detect and remove them [Cite](https://openaccess.thecvf.com/content_cvpr_2018/papers/Liao_Defense_Against_Adversarial_CVPR_2018_paper.pdf)
- Generate adversarial examples by spatially transforming natural images. Resulting faces completely lie on the manifold of natural images, making them extremely hard for defense methods to detect. [Cite](https://arxiv.org/pdf/1801.02612.pdf)
- Enormous amounts of makeup (harvey2017cv) or [wearing carefully crafted accessories](https://dl.acm.org/doi/abs/10.1145/2976749.2978392?casa_token=aBpzAMJk-j4AAAAA:8A2EVzlMWOY5rOyB0tDuTwNlx0VGjIGexosoNSclrV6cv5JKmDtYgf3fquqAiG6dkRd3s5mYCKeV) can fool some facial detection methods.

#### Analysis

- Some of the generated faces do not look natural. Figures 4 and 5 in the paper shows generated faces.

Citation: Dabouei, Ali, et al. "Fast geometrically-perturbed adversarial faces." 2019 IEEE Winter Conference on Applications of Computer Vision (WACV). IEEE, 2019.

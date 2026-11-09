+++
title = "Accessorize to a Crime: Real and Stealthy Attacks on State-of-the-Art Face Recognition"
date = "2016-10-01"
paper_url = "https://dl.acm.org/doi/abs/10.1145/2976749.2978392"
tags = ["Adversarial", "Misclassification", "Perturbation", "Physical"]
citations = 624
+++

#### Summary

They developed a physical overlay on glasses that can cause an individual to either dodge a facial recognition system (FRS) or impersonate someone else. This overlay would be added to "nerd" glasses. Their attacked used a white-box approach and attacked a FRS developed by [Omkar et al.](https://ora.ox.ac.uk/objects/uuid:a5f2e93f-2768-45bb-8508-74747f85cad1), which consisted of 39 layers. Their loss function consisted of three parts. The first part accounted for miss-classification and can be found in section 4.2 in the paper. The second part is in section 4.3.3 and accounted for smoothness of the overlay. The third part accounted for natural colors, that could be found in the real world. It also accounted for printer error. This can be found in section 4.3.4 of the paper.

#### Notes

- Used 3 different DNNs for facial recognition. DNN<sub>A</sub> was trained on 2622 celebrities with 1000 images per celebrity. DNN<sub>B</sub> was trained with 10 people. DNN<sub>C</sub> was trained on 143 people. Used transfer learning to train B and C using DNN<sub>A</sub>
- Used Gradient Descent to minimize the loss functions
- At least 80% dodging success rate

#### Analysis

- The glasses overlay took up roughly 6% of the face. This is good, however, they only trained the FRS from a single angle, with a single (neutral) facial expression, with indoor lighting conditions.
- Their system worked quite well when the highest probability class was always chosen, but when the probability had to be above some threshold (0.85 or 0.9), then accuracy varied dramatically.

Citation: Sharif, Mahmood, et al. "Accessorize to a crime: Real and stealthy attacks on state-of-the-art face recognition." Proceedings of the 2016 acm sigsac conference on computer and communications security. 2016.

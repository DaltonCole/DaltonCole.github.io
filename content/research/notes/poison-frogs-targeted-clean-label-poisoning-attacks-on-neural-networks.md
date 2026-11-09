+++
title = "Poison Frogs! Targeted Clean-Label Poisoning Attacks on Neural Networks"
date = "2018-11-10"
paper_url = "http://papers.nips.cc/paper/7849-poison-frogs-targeted-clean-label-poisoning-attacks-on-neural-networks.pdf"
tags = ["Adversarial", "Data-Poisoning", "Perturbation"]
citations = 145
math = true
+++

#### Summary

Performed a data-poisoning attack, where the goal was to misclassify a certain target image to be classified as a base image (a goal label), without degrading the overall accuracy of the system. An image from the base class would be perturbed so it carried features from the target image, thus moving the decision boundary for the target image towards the base class. Two different training strategies were used. When transfer learning was used (froze all but the last layer), they achieved 100% attack success rate with a single photo. When end-to-end learning was used (which is more realistic), they achieved up to a 70% success rate, however, 50 poisoned images were required plus a 30% opacity watermark was required plus they had to pick the lowest confident images in the base class to perturbed. All attacks consisted of a single target image to be labeled as the base class.

#### Notes

- Attack Types:
  - Evasion Attacks: Happen at test time. A clean target instance is modified to avoid detection or be misclassified.
  - **Data Poisoning Attacks**: Happen at training time. Aim to manipulate the performance of a system by inserting carefully constructed poison instances into the training data.
- Assumes no knowledge of the training data, but knowledge of the model and its parameters
- Goal: misclassify a certain target image with a base (goal) label while not degrading the overall accuracy of the classifier. Does this via perturbing an image from the base class to contain features from the target image.
- Equation 1 shows the minimization algorithm to generate the poisoned image.
- $$\boldsymbol{p} = \underset{x}{argmin} \left\Vert f(\boldsymbol{x}) - f(\boldsymbol{t}) \right\Vert^{2}_{2} + \beta \left\Vert \boldsymbol{x} - \boldsymbol{b} \right\Vert^{2}_{2}$$
- $f(\boldsymbol{x})$ is the function that propagates x through the network at returns the feature vector prior to the soft-max layer. The $\beta$ term makes p appear as the base class to human observers. The first term causes the poisoned instance to be moved towards the target instance in feature space.
- This can be thought of as creating a backdoor to the base class for a specified target.
- Transfer Learning
  - Attack where, during training, all but the last layer is frozen.
  - Uses the InceptionV3 network and dog-vs-fish dataset
  - Figure 1.a demonstrates the poisoned samples.
  - A single poisoned image was required to misclassify the target with high confidence 100% of the time (>.95 confidence in the binary case)
- End-To-End Learning
  - Realistic. The entire network is trained.
  - Observation: retraining caused the lower-level feature extraction kernels in the shallow layers to return the poisoned instance to the base class.
  - Added watermarks to increase misclassification accuracy. Watermark had 20-30% opacity. Still only had 70% attack accuracy.
- Angular Deviation (statistic):
  - "The degree to which retraining on the poison instance caused the decision boundary to rotate to encompass the poison"
  - Looks at the weight vectors of a clean and poisoned network and measures the angular difference.
  - There is a large difference between the clean and poisoned networks when transfer learning was applied. A very small difference with end-to-end. This is shown in figure 2.
- Figure 3 visualizes the feature space change during the attack.

#### Analysis

- The high accuracy attack is unrealistic while the realistic attack performs much worse.

Citation: Shafahi, Ali, et al. "Poison frogs! targeted clean-label poisoning attacks on neural networks." Advances in Neural Information Processing Systems. 2018.

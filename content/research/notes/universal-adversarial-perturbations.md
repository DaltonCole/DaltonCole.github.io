+++
title = "Universal adversarial perturbations"
date = "2017-03-09"
paper_url = "https://arxiv.org/pdf/1610.08401.pdf"
tags = ["Adversarial", "Misclassification", "Perturbation"]
citations = 891
math = true
+++

#### Summary

They proposed an algorithm to find a universal adversarial perturbation to natural images in order to misclassify them. By using a single perturbation filter, they were able to misclassify between 77.8% and 93.3% of samples depending on the classification model (seen in Table 1 of the paper). The minimization algorithm can be seen in equation 1 and algorithm 1 in the paper. They iteratively went through every sample in the dataset and altered their current perturbation in order to move the current image + perturbation to the decision boundary in a minimizing fashion.

They also looked at how effective applying the same filter to a different neural network would be. As seen in Table 2 of the paper, their accuracy ranged from 39.2 to 74.0%, which is amazing. In Figure 5, they also noticed that when the sample images where given to the minimization function in a different order, different perturbations would result. The perturbations were less than 0.1 similar at times, meaning, many universal perturbations could be generated.

As a defense mechanism, they attempted to train the neural network with perturbed images. When 50% of the dataset was perturbed, after 5 epochs, the perturbation attack went from 93.7% accurate to 76.2% accurate. With altering the percent of perturbed images and number of epochs, they were NOT able to reduce the accuracy of the attack.

Finally, they observed that a few labels were dominating the misclassification, i.e. many images were being classified as a few labels. This can be seen in Figure 7 of the paper.

#### Notes

- Fantastic paper
- Used both $l_2$ and $l_\infty$ norms to measure the perturbation. $l_\infty$ norm resulted in higher misclassification accuracy.
- "as it shows that we can fool a large set of unseen images, even when using a set X containing less than one image per class!"

#### Analysis

- Did not find a viable defense strategy
- The perturbed images were "quasi-imperceptible" to the human eye, meaning, they visually looked altered, but barely. The Balloon example in Figure 1 is a good illustration of this.

Citation: Moosavi-Dezfooli, Seyed-Mohsen, et al. "Universal adversarial perturbations." Proceedings of the IEEE conference on computer vision and pattern recognition. 2017.

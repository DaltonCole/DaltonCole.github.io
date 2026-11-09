+++
title = "Neural Cleanse: Identifying and Mitigating Backdoor Attacks in Neural Networks"
date = "2019-09-16"
paper_url = "http://people.cs.uchicago.edu/~huiyingli/publication/backdoor-sp19.pdf"
tags = ["Backdoor", "Detection"]
citations = 123
+++

#### Summary

Learned the perturbation used to generate a backdoor. They were then able to remove the back door from the neural network.

Their detect and identify steps were combined. To identify a backdoor, they found a minimal perturbation filter required to transform all samples from all other labels to the targeted label. They did this for every label. They then ran outlier detection on these filters. If a filter was determined to be an outlier, then it was marked as a backdoor.

To mitigate backdoors, they applied two different approaches. They empirically noticed that the top 1% of neuron activation's heavily correlated to the backdoor. They set neurons that were highly correlated to the backdoor in the second to last layer of the network to 0. Their second approach used unlearning. They used the reversed trigger (trigger inverted) and re-trained the network using those samples. They only used 10% of the training date for unlearning, and used the reversed trigger on 20% of these samples.

#### Notes

- They consider backdoor attacks to be separate from adversarial poisoning attacks.
- Three goals: detect, identify, and mitigate a backdoor
- They were able to find a smaller backdoor trigger than the trigger that was initially used. See figure 7 in paper.

#### Interesting References

- [Liu et al.](https://docs.lib.purdue.edu/cgi/viewcontent.cgi?article=2782&context=cstech) made a Trojan attack by inducing the maximum response of specific internal neurons in a DNN

#### Analysis

- If every label had a backdoor, the outlier detection will not detect any backdoor.
- Their experiment added a white square to the bottom right of the image to generate a backdoor. Not realistic.
- Max trigger size was 18-39% of an image.
- Was resource intensive, but they provided a less resource intensive model as well.

Citation: Wang, Bolun, et al. "Neural cleanse: Identifying and mitigating backdoor attacks in neural networks." 2019 IEEE Symposium on Security and Privacy (SP). IEEE, 2019.

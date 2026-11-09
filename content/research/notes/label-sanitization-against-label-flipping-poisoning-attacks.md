+++
title = "Label Sanitization against Label Flipping Poisoning Attacks"
date = "2018-10-02"
paper_url = "https://arxiv.org/pdf/1803.00992.pdf"
tags = ["Data-Poisoning", "Defense", "Label-Flipping"]
citations = 39
+++

#### Summary

#### Notes

- Defined "optimal poisoning attacks" pretty well at the start of section 2.
- Attack Method:
  - "worst-case scenario". Assumes full white-box access to everything. Model + dataset + separate validation dataset from the same distribution of the dataset. Admits that this is unrealistic.
  - Used a binary linear classifier, but is applicable to a multi-classification problem as well.
  - Attacker wishes to maximize the loss function
  - Algorithm 1 gives the attack. Greedily selects examples from the training dataset that maximizes validation loss.
- Defense Method:
  - Label sanitization based
  - Considers flipped samples as outliers
  - Algorithm 2 gives the defense. Uses k-NN. Given some threshold, if the number of samples in the majority in the k-NN cluster are greater than the threshold, the minority's labels are changed to match the majority.
  - "Poisoning points that are far from the decision boundary are likely to be relabelled, mitigating their malicious effect on the performance of the classifier"
  - Admits that genuine samples will be relabeled when two classes overlap
- Experimental Results:
  - Used 3 datasets: BreastCancer (30 features), MNIST (1 vs 7) (784 features), SpamBase (54 features)
  - When 20% of the data is poisoned, the classification error increased between 2.8 and 6 times.
  - Defensive strategy does slightly degrade the performance of the classifier
  - Figure 1 shows the results of the attack and defense (LF = attack, kNN = defense)
  - Figure 2 shows the impact of k in k-NN and the threshold value.
    - More samples per cluster leads to a better defense strategy
    - A threshold of 50% lead to the best defense strategy. I.e., the minority is always relabeled to be the same as the majority.

#### Interesting References

- "manipulate samples at test time to evade detection or inject malicious data into the training set to poison the learning algorithm" [LINK](https://dl.acm.org/doi/abs/10.1145/2046684.2046692?casa_token=22_s9SUosbUAAAAA:1vcC8hHGxEb5xUSNzNu3q2CLPMBYj9g2YLAPEUGd8waV04uv0JzNz1K2NLRACXcOSePHNCw7nIWe)
- Data poisoning attacks are very relevant as an emerging security threat for data-driven technologies. [LINK](https://drops.dagstuhl.de/opus/volltexte/2013/4356/)
- Label flipping attacks can degrade the performance of deep learning based networks significantly. [LINK](https://arxiv.org/abs/1611.03530)
- Label flipping attack paper. Adversary selects a subset of the training data to maximize the error. [LINK](http://proceedings.mlr.press/v20/biggio11.html)
- Defense Strategies:
  - "Defences against optimal poisoning attacks typically consist either in identifying malicious examples and discarding them from the training data or they require to solve some robust optimization problem"
  - Outlier detection method for label flipping attacks that identities and removes suspicious samples. [LINK](https://arxiv.org/abs/1802.03041)
  - Solve optimization problem. Assumes knowledge of the amount of poisoned data. [LINK](https://proceedings.neurips.cc/paper/2014/file/6cdd60ea0045eb7a6ec44c54d29ed402-Paper.pdf)
  - Solve optimization problem. Requires an estimation of the amount of poisoned data. Iteratively removes poisoned data based on current error estimates. [LINK](https://ieeexplore.ieee.org/abstract/document/8418594?casa_token=ki8JcLAh3VkAAAAA:M-ADAEdIvYY-Z2YIUA35NWtyoNuYnoSboocxoYClFGp7rMS4MLRHFtch3ND_d-IjylnvkpVbc8tG)
  - Evaluates the degree each training sample affects the learning algorithm's performance. Samples that negatively affect the loss function are discarded. Does not scale well. [LINK](https://static.usenix.org/event/leet08/tech/full_papers/nelson/nelson.pdf)
  - "detect the most influential training points". Similar to ^, but scales better. Does not retrain the model. [LINK](http://proceedings.mlr.press/v70/koh17a)

Citation: Paudice, Andrea, Luis Muñoz-González, and Emil C. Lupu. "Label sanitization against label flipping poisoning attacks." Joint European Conference on Machine Learning and Knowledge Discovery in Databases. Springer, Cham, 2018.

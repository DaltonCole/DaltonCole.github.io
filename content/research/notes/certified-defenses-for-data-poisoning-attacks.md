+++
title = "Certified Defenses for Data Poisoning Attacks"
date = "2017-06-01"
paper_url = "http://papers.nips.cc/paper/6943-certified-defenses-for-data-poisoning-attacks.pdf"
tags = ["Data-Poisoning", "Defense"]
citations = 191
+++

#### Summary

Approximate a loss upper bound for data poisoning attacks that have the goal of maximizing loss. Assumes the defense uses outlier removal. Other assumptions are: (1) train and test are correlated and (2) outliers in the clean data do not have a strong effect on the model. [GitHub](https://github.com/kohpangwei/data-poisoning-release)

#### Notes

- They test using binary SVM models, but their method works on multi-class problems using any method
- Two different defense settings discussed:
  - Fixed defense. There is an oracle that knows the exact distribution of D<sub>c</sub> and can choose as feasible dataset from that knowledge (not implementable in practice)
  - Data-dependent defense. The fixed feasible (F) dataset is dependent on D<sub>c</sub> ∪ D<sub>p</sub>. This type of defense opens up a game theory type of attack where the attacker messes with F instead of just the loss function
- The defense's goal is to minimize the loss function given in equation 1 (minimize the loss given the entire dataset intersected with the feasible set)
- They implemented two different methods of outlier removal (equation 2):
  - F<sub>sphere</sub>, where points beyond a radius are marked as outliers
  - F<sub>slab</sub>, where points too far from a line are marked as outliers
- They show that the optimal attack strategy is a max-min objective function (shown in equation 4).
- Equation 5 shows the equation for an optimal defense where all n points are at a single, maximized loss, point. This is essentially equation 4 with min-max instead of max-min
- Equation 8 shows when the poisoned points must follow a probability distribution π<sub>p</sub> as opposed to being concentrated at a single point.
- **Results**
  - Oracle Defense
    - Figure 2.c shows that their attack outperforms label flipping and gradient descent attacks, thus the defense holds for these sub-optimal attack methods.
    - As soon as ε (the min-max weight towards D<sub>p</sub>), the model chooses to fit the attack points
    - The attacks generally like to tightly concentrate around a single point at the boundary of F (thus moving the centroid considerably)
  - Data-Dependent Defense
    - Figure 4 shows this defense. It performs roughly 10x worse than the oracle defense.
    - The attacker can choose attacking locations that lie substantially outside the original (clean) feasible set (F)
- Main requirement for their framework requires l(θ, x, y) to be maximized over all feasible x and y.
- Reported a 11% reduction in test accuracy when the attacker is allowed 3% training set modifications.

#### Interesting References

- Lots, check related works section (section 6)

#### Good Quotes

- "..the most critical ingredient of all – the training data – comes directly from the outside world."

Citation: Steinhardt, Jacob, Pang Wei W. Koh, and Percy S. Liang. "Certified defenses for data poisoning attacks." Advances in neural information processing systems. 2017.

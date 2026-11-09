+++
title = "Towards Poisoning of Deep Learning Algorithms with Back-gradient Optimization"
date = "2017-11-03"
paper_url = "https://dl.acm.org/doi/pdf/10.1145/3128572.3140451"
tags = ["Adversarial", "Data-Poisoning"]
citations = 207
math = true
+++

#### Summary

This paper provides a lot of good definitions on data poisoning attacks in section 2. Section 3 outlines a poisoning attack involving back-gradient optimization, while section 4 covers the experimental analysis analyzing spam/malware detection (DNN, Logistic Regression, and Adaline) and handwritten digit recognition (CNN). They show that poisoning attacks crafted for one method can be transferred to other methods, but with much reduced performance.

#### Notes

- **Threat Model**
  - Poisoning Attack: Attack at training time
  - Evasion Attack: Attack at testing time
  - Attacker's Goal
    - Security Violation
      - Integrity Violation: Evade detection without compromising normal system operations
      - Availability Violation: Compromise normal system functionality (increase classification error)
      - Privacy Violation: Gains private information about the system, users, or data
    - Attack Specificity
      - Targeted or indiscriminate misclassification
    - Error Specificity
      - Specific: Samples misclassified to a targeted class
      - Generic: Samples misclassified to any class
  - Attacker's Knowledge
    - Perfect-Knowledge Attacks
      - Worst-case
      - Attacker knows about the:
        - Training Data
        - Feature Set
        - Learning Algorithm and Objective Function
        - Trained Parameters
    - Limited-Knowledge Attacks
      - Attacker has some knowledge about the system
  - Attacker's Capability
    - Attack Influence
      - Causative (poisoning): Influence both training and testing data
      - Exploratory (evasion): Influence testing data
    - Data Manipulation Constraints
      - Platform dependent
  - Attack Strategy
    - Alter the dataset to maximize the objective function
  - Poisoning Attack Scenarios
    - Error-Generic Poisoning Attacks
      - Denial of Service based attack. Just want as much misclassification as possible.
      - Inner Function: Use parameters $\hat{w}$ that minimize the loss function
      - Outer Function: Maximize the loss function
    - Error-Specific Poisoning Attacks
      - Cause a specific misclassification
      - Inner Function: Same as above
      - Outer Function: Minimize the loss function

Citation: Muñoz-González, Luis, et al. "Towards poisoning of deep learning algorithms with back-gradient optimization." Proceedings of the 10th ACM Workshop on Artificial Intelligence and Security. 2017.

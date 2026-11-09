+++
title = "Exploring Connections Between Active Learning and Model Extraction"
date = "2019-11-20"
paper_url = "https://arxiv.org/pdf/1811.02054.pdf"
tags = ["Model-Extraction"]
citations = 10
math = true
+++

#### Summary

Formalized model extraction and possible defense strategies. Drew parallels between model extraction and established area of active learning. Looked into Machine Learning-as-a-Service (MLaaS) and the associated costs of reverse engineering a model.

#### Notes

- Section 2.1 defines passive learning. Section 2.2 defines active learning
- PAC (probability approximately correct) learning has access to a distribution of the dataset. They then query the model to generate the hypothesis $\hat{f}$ such that the expected loss from the dataset is low.
- The active learning version of PAC selects items from the dataset.
  - Stream-Based Sampling: Select samples sequentially, once you pass a sample, you don't go back to it
  - Pool-Based Sampling: Select samples from a pool of samples (better).
- Query Synthesis Active Learning: Pool based sampling but can generate samples independent of the given dataset distribution (Most realistic for model extraction)
- Active learning needs to be able to evaluate the usefulness of an unlabeled instance.

#### Interesting References

- Modifies input features resulting in misclassification by DNNs (data poisoning) - [LINK](https://dl.acm.org/doi/abs/10.1145/3052973.3053009?casa_token=M0yjrvwiAvEAAAAA:RpemfyPPDIxiseSP7HSwoaZWZGgPXcrykAZXvQbgUub77hVBgxa22I8UOMI75eovBeDnytq9_Kej)
- Looks at: mislabeling training data, changing rewards in the case of reinforcement learning, and modifying the samples mechanisms (data poisoning paper) - [LINK](https://ieeexplore.ieee.org/abstract/document/7467366/?casa_token=HWbv_4z9VXEAAAAA:jtgHkRoHvIuDekfre4QdG4tgJr_DaQpFWIWW4GsA5YoBprSFkI8PR9O86dqtxDopb6wXYtALujQc)

Citation: Chandrasekaran, Varun, et al. "Exploring connections between active learning and model extraction." 29th {USENIX} Security Symposium ({USENIX} Security 20). 2020.

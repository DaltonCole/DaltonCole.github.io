+++
title = "FaceNet: A Unified Embedding for Face Recognition and Clustering"
date = "2015-03-12"
paper_url = "https://www.cv-foundation.org/openaccess/content_cvpr_2015/papers/Schroff_FaceNet_A_Unified_2015_CVPR_paper.pdf"
tags = ["Framework"]
citations = 5819
+++

#### Summary

Learns a mapping from face images to a compact Euclidean space (128 dimensions) where distances directly correspond to a measure of face similarity. Uses a deep convolutional network. Uses online triplet mining method where each triplet consists of a base, one positive example, and one negative example.

#### Notes

- Uses the Labeled Faces in the Wild (99.63% accuracy) and [YouTube Faces DB](https://www.cs.tau.ac.il/~wolf/ytfaces/") datasets (95.12% accuracy).
- Uses squared L2 distances in the embedding space
- Recognition is a k-NN classification problem (neural network output)
- Scale and translation are performed to thumbnails
- Employers hard-positive mining techniques to encourage spherical clusters for the embedding of a single person (cor clustering)
- Purely data driven method which learns its representation directly from the pixels of the face.
- Uses the [Zeiler & Fergus](https://arxiv.org/pdf/1311.2901.pdf) and [Inception model](https://www.cv-foundation.org/openaccess/content_cvpr_2015/papers/Szegedy_Going_Deeper_With_2015_CVPR_paper.pdf)
- Triplet Loss: Minimize the distances between an anchor (base sample) and a positive sample, and maximize the distance between the anchor and the negative sample.
- 22 layers. Outlined in Table 1 in the paper.
- Trained neural network using Stochastic Gradient Descent with standard back propagation and AdaGrad
- Inception drastically reduced model size

#### Analysis

- Takes 1000 to 2000 hours to train
- Requires a tight crop of the face area
- Uses a proprietary face detector to get best results

Citation: Schroff, Florian, Dmitry Kalenichenko, and James Philbin. "Facenet: A unified embedding for face recognition and clustering." Proceedings of the IEEE conference on computer vision and pattern recognition. 2015.

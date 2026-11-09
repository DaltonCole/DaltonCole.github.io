+++
title = "FaceShifter: Towards High Fidelity And Occlusion Aware Face Swapping"
date = "2020-09-15"
paper_url = "https://arxiv.org/pdf/1912.13457.pdf"
tags = ["Dataset", "Deepfake"]
citations = 25
+++

#### Summary

Built a great deepfake generation framework that would be placed in the second generation of deepfake datasets. Generated a dataset where the real dataset was from the 1,000 YouTube videos FaceForensics++ published.

Framework consists of two parts. The first part is a normal GAN based approach to generating deepfakes with a different multi-layer architecture. The second part made sure facial obstructions in the source would remain in the deepfake.

#### Notes

- Introduction:
  - GAN-based methods work better than 3D modeling methods
  - Deepfakes need to consider the lighting (direction, intensity, and color) and the resolution of the target. Poisson blending, the normal blending technique used by deepfake generators, does not achieve this. Blending removes the peripheral attributes.
- Methods:
  - See Figure 3 for an overview of the first stage of the network.
  - **Adaptive Embedding Integration Network (AEI-Net)**
    - Used to generate a high fidelity face image
    - Consists of three parts:
      - Identity Encoder: Extract identity from the source image.
        - Believes lots of 2D face data is better than a 3D-based model
      - Multi-Level Attributes Encoder: Extracts attributes of the target image
        - Uses a multi-level feature map to better preserve spatial information such as pose, expression, lighting, and background (as opposed to a single feature vector)
      - Adaptive Attentional Denormalization (AAD) Generator: Generates the swapped face image
        - Previous methods simply used a concatenation layer, which leads to a blurry image
        - Used an attention mechanism to focus on intensifying parts of the face, such as the eyes, mouth, and face contour
        - Specifically used an attentional mask
    - Used adversarial training for the AEI-Net (GAN based training)
  - **Heuristic Error Acknowledging Refinement Network**
    - Used a heuristic method for facial obstructions
    - Since obstructions generally disappear in reconstructed images, they used the error between the reconstructed image and its input to locate the face obstructions
- Experiments:
  - Qualitative: Compared with FaceForensics++ dataset among others. FaceShifter looks the best (Figure 5, 6, 7)
  - Quantitative: Based on L-2 distances, FaceShifter performed best on ID retrieval and expression and near best on pose (Table 1)
  - Human Evaluation: FaceShifter won. See table 2.

#### Analysis

- Deepfakes look really good
- Would put this dataset as a second generation dataset

Citation: Li, Lingzhi, et al. "Faceshifter: Towards high fidelity and occlusion aware face swapping." arXiv preprint arXiv:1912.13457 (2019).

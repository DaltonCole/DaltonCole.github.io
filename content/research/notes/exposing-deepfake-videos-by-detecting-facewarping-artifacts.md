+++
title = "Exposing DeepFake Videos By Detecting Face Warping Artifacts"
date = "2019-05-22"
paper_url = "https://arxiv.org/pdf/1811.00656.pdf"
tags = ["Deepfake", "Detection"]
citations = 86
+++

#### Summary

Generated a dataset to train deepfake detectors that did not consist of any deepfakes. They cropped the face, applied Gaussian blur to blur the face, affine (scaling, rotation, shearing) warped the face back to normal (thus creating additional artifacts). The dataset proved to train the given CNN models well enough to outperform other models that were trained on actual deepfakes.

#### Analysis

- Using a generated dataset that does not contain deepfakes is an interesting idea, however, as deepfakes improve, this method will become less viable
- For testing, they used fairly small datasets.

#### Notes

- DeepFake algorithms output faces of a fixed size, thus they must undergo an affine wrapping to bring the face back to its original quality
- The face alteration generally leaves behind artifacts or makes the image less crisp/blurrier
- They took advantage of this to create a deepfake dataset that did not require running a deepfake model.
- Training Dataset
  - Used 24,442 JPEG images as the positive examples, which were also used to generate the negative examples
  - Figure 2 highlights how they generated their training dataset
    1. They aligned the face with different scales (scale the image down X%)
    2. Applied Gaussian blur (made the image a bit blurry)
    3. Warped the image back to the original dimensions
  - Figure 3 highlights how they wrap different parts of the face
  - 
- Model
  - Generated "regions of interest" (RoI) via landmarks in the face. 10 RoIs were used for each image
  - Used four different CNN models: VGG16, ResNet50, ResNet101, and ResNet152.
  - Averaged the predictions of all RoIs probabilities to determine the final probability of the image being fake or real
- Validation Datasets
  - Used the UADFV dataset consisting of 49 real and 49 fake videos each lasting ~11 seconds. ResNet50 had the highest Area Under Curve (ROC) at 0.954 per frame. ResNet101 had .991 per video
  - Used the DeepfakeTIMIT dataset consisting of a few low and high quality videos. ResNet50 had the highest AUC per frame at .999 for low quality videos and .932 for high quality videos
- ResNet50 outperformed other models from other papers (Table 1) despite being trained on a non-deepfake dataset.

Citation: Li, Yuezun, and Siwei Lyu. "Exposing deepfake videos by detecting face warping artifacts." arXiv preprint arXiv:1811.00656 (2018).

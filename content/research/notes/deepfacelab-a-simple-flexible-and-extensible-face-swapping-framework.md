+++
title = "DeepFaceLab: A simple, flexible and extensible face swapping framework"
date = "2020-05-20"
paper_url = "https://arxiv.org/pdf/2005.05535.pdf"
tags = ["Deepfake", "Framework"]
citations = 2
+++

#### Summary

DeepFakeLab (DFL) is a framework that produces deepfakes. The framework is able to produce high quality image deepfakes and comes with many customization abilities.

#### Notes

- DeepFakeLab is an open source pipeline for generating DeepFakes. The pipeline consists of a: face detector module, face recognition module, face alignment module, face parsing module, and a face blending module
- Leras: "Lighter Keras". DeepFakeLab is written in Tensorflow (keras had too much overhead in their opinion)
- Only requires a source and a destination folder to make deepfakes.
- Pipeline consists of three phases: extraction, training, and conversion
  - Extraction
    - Face Detector: Uses [S3FD](https://openaccess.thecvf.com/content_ICCV_2017/papers/Zhang_S3FD_Single_Shot_ICCV_2017_paper.pdf) as the default face detector
    - Face Alignment:
      - 
      - Heatmap-based facial landmark algorithm 2DFAN
      - PRNet with 3D face priori information. Option to smooth facial landmarks of consecutive frames (should increase temporal detection resilience?)
      - Uses the Umeyama method to calculate a similarity transformation matrix for face alignment. DFL supports front view and left/right side views, calculated via the Euler angle of the obtained facial landmarks
      - Face Segmentation: Removes facial obstructions (such as hair) via TernausNet
  - Training
    - Figure 3 has a network overview.
    - Loss function: DSSIM (structural dissimilarity) + Mean Squared Error (MSE)
      - DSSIM generalizes human faces faster than MSE
      - MSE has better clarity
    - Figure 4 outlines their srs2dst (source to destination) method.
      - Equation 1 shows their blending algorithm. They use the re-aligned generated face, the target face, and a target mask
      - Equation 2 is for Poisson blending optimization
      - To sharpen the image, they use a pre-trained face super resolution neural network called FaceEnhancer.
- For HD images, they use pixelshuffle (depth2space) to perform upsampling.

#### Interesting References

- [StyleGAN](https://github.com/NVlabs/stylegan) and [StyleGAN2](https://github.com/NVlabs/stylegan2) use GANs and [style transfer](https://towardsdatascience.com/style-transfer-with-gans-on-hd-images-88e8efcf3716) to generate new artificial faces.

Citation: [DeepFakeLab](https://github.com/iperov/DeepFaceLab)

+++
title = "FaceForensics++: Learning to Detect Manipulated Facial Images"
date = "2019-08-26"
paper_url = "https://arxiv.org/pdf/1901.08971.pdf"
tags = ["Dataset", "Deepfake", "Detection", "Facial-Reenactment", "Survey"]
citations = 138
+++

#### Summary

Created a large video database consisting of 1000 videos each manipulated (automatically) via Deepfakes, Face2Face, FaceSwap, and NeuralTextures. Used five different state of the art detection methods to see how well they were able to detect real vs fake images (frames). XceptionNet outperformed the other methods. Each method did worse on low quality images and best on raw quality images (i.e. compression is hard).

#### Notes

- Made a dataset consisting of 1000 video sequences that have been manipulated with via Deepfakes, Face2Face, FaceSwap, and NeuralTextures. [LINK](https://github.com/ondyari/FaceForensics)
- There are currently two types of facial manipulation methods: Facial expression manipulation and facial identity manipulation. Figure 2 gives a good example.
  - Identity Swap: The face from one person is transposed onto another person
    - **FaceSwap**
      - "Graphics-based approach to transfer the face region from a source video to a target video"
      - Finds facial landmarks → Extract face → Back projected to a target image by minimizing landmark distances → Blend image and apply color correction
    - **DeepFakes**
      - FakeApp and [faceswap](https://github.com/deepfakes/faceswap) (DeepFake Method) are public implementations
      - Uses neural networks: single encoder and two decoders
      - Face detector used to crop and align images. [Poisson](https://www.cs.jhu.edu/~misha/Fall07/Papers/Perez03.pdf) image editing is used to blend the image
  - Facial Reenactment: The facial expression from one person is transposed onto another. The original identity if maintained, only the expression changes.
    - **Face2Face**
      - A "dense reconstruction" of the face is generated via following the face through multiple frames of a video. This is "used to re-synthesize the face under different illumination and expressions"
    - **[NeuralTextures](https://dl.acm.org/doi/pdf/10.1145/3306346.3323035)**
      - "Uses original video data to learn a neural texture of the target person"
      - Uses a unique model for every manipulation. Proves the most difficult for the below detectors.
- Forgery Detection - A per-frame binary classification problem ("Real" or "Fake")
  - Human
    - Figure 4 highlights their results when humans classify images as real or fake. Facial reenactment was really hard to detect (around 50% accuracy, i.e. random guessing). Identity swap was much easier to detect with generally >70% accuracy. It was generally harder to tell the difference between real and fake with lower quality images
  - Automated
    - Figure 6 summarizes the accuracy results
    - Steganalysis Features
      - Based on a method by [Fridrich et al.](https://ieeexplore.ieee.org/document/6197267) which won the first [IEEE Image Forensic Challenge](https://ieeexplore.ieee.org/abstract/document/7026072?casa_token=S6Vbh_TBdrYAAAAA:tv1JkXr-jGguT6IJ9Txg_iVOlW8SPhFU1lmsoRXcfU353nsBcN7GaZdzePFBlG1k3H-24Y4CsCor). Uses 162 features derived from a 128x128 central crop-out image of the face, which is then feed into a SVM
      - Preforms really well on raw image, but struggles with compressed images
    - [Cozzolino et al.](https://dl.acm.org/doi/pdf/10.1145/3082031.3083247)
      - Used the same features from the "Steganalysis Features" classifier, but used a CNN-based network instead of a SVM.
      - Preforms better than the SVM Steganalysis features method, but still stugles with low quality videos
    - [Bayar and Stamm](https://dl.acm.org/doi/pdf/10.1145/2909827.2930786)
      - CNN based approach that uses constrained convolutional layers
      - Better than the previous two methods. Still struggles with low quality images
    - [Rehmouni et al.](https://ieeexplore.ieee.org/abstract/document/8267647?casa_token=Zme-MdYCwHcAAAAA:XuWqdTEPb8j3QR-UaBkDmmJqWQjg26tGkdPt8hYr004xa5qn2i82H2v7bfgK9cwOzkE5kcdXSAUm)
      - CNN based approach. Computes some statistics.
      - Roughly the same results as Cozzolino et al.'s method
    - [MesoInception-4](https://ieeexplore.ieee.org/abstract/document/8630761?casa_token=7BGihSwA2KMAAAAA:dA17qHWCAisibseWERfeuZkZ8eBoBSjk_Ggec6BhZ4swyvAmjb5SM18rV1S4MkRN_4gXu0-4Y1C4)
      - CNN based approach inspired by [InceptionNet](https://arxiv.org/abs/1602.07261)
      - Has two inception modules and two classical convolution layers
      - Better results than the previous 4 methods (generally)
    - [XceptionNet](https://openaccess.thecvf.com/content_cvpr_2017/papers/Chollet_Xception_Deep_Learning_CVPR_2017_paper.pdf)
      - Traditional CNN trained on ImageNet. Based on separate convolutions with residual connections. They re-fitted this network for their purposes by transferring most of the network and replacing the final connected layer with two outputs.
      - Best results. Preforms well with low quality images.
    - Figure 8 shows how important a large dataset is.

#### Interesting References

- Face2Face uses a state-of-the-art face tracking method. Good for extracting the face region in an image.

Citation: Rossler, Andreas, et al. "Faceforensics++: Learning to detect manipulated facial images." Proceedings of the IEEE International Conference on Computer Vision. 2019.

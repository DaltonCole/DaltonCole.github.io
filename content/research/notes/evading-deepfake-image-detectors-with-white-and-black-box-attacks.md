+++
title = "Evading Deepfake-Image Detectors with White- and Black-Box Attacks"
date = "2020-04-01"
paper_url = "https://openaccess.thecvf.com/content_CVPRW_2020/papers/w39/Carlini_Evading_Deepfake-Image_Detectors_With_White-_and_Black-Box_Attacks_CVPRW_2020_paper.pdf"
tags = ["Adversarial", "Black-Box", "Deepfake", "Detection", "Perturbation"]
citations = 1
+++

#### Summary

Created 5 different adversarial perturbation methods to attack deepfake detectors, primarily Frank et al's detector. The first two attacks focused on only changing the least significant bit in every pixel, the third created a universal patch, the fourth used a universal (single) low-level attribute vector of ProGAN to generate adversarial deepfakes, the fifth was a black-box method where adversarial perturbation was learned from one network, and those samples were tested on the goal network. Figure 3 shows the last 4 attack's results using ROC curves.

#### Notes

- Uses two detectors: [Wang et al.](https://openaccess.thecvf.com/content_CVPR_2020/papers/Wang_CNN-Generated_Images_Are_Surprisingly_Easy_to_Spot..._for_Now_CVPR_2020_paper.pdf) and [Frank et al.](https://arxiv.org/pdf/2003.08685.pdf) with a focus on Wang's.
  - Wang et al.:
    - Based on ResNet-50
    - Pre-trained on ImageNet with 720,000 training and 4,000 validation images (half rea, half fake using ProGAN)
    - Dataset had been augmented by spatial blurring and JPEG compression
    - Accuracy is tested against 10 different generators
    - Robust to spatial blurring and JPEG compression
  - Frank et al.
    - "The authors argue that GAN synthesized-images have a command spatial frequency artifact that emerges from image upsampling..." → temporarily similar image artifacts
- Looked at five different attack strategies:
  - Distortion-Minimizing Attack
    - Limited their attack to only flipping the lowest bit of each pixel → the maximum perturbation to any pixel is 1/255
    - Equation 2 shows their optimization function. They try to minimize the perturbation while minimizing the probability of the new image being labeled fake
    - Figure 2.a shows the results of Fake images being marked as Real. Half of all faked images are classified as real by flipping the least significant bit in 1% of pixels. At 11% of pixel flips, nearly 100% of fakes are labeled real
    - Figure 2.b shows the results of Real images being marked as Fake. This is the only attack that tests both scenarios. They find that it takes 7% of pixels being flipped to get 50% of real images to be labeled fake, as opposed to the above 1% findings for fake to real. Easier to change computer generated images? Are the discriminators really learning real photos while kind of guessing at fake ones? Looking at how certain the discriminators are of real images would be an interesting study.
  - Loss-Maximizing Attack
    - Similar but simpler than the Distortion-Minimizing Attack. Equation 3 shows the optimization function. They minimize the perturbation p-norm as opposed to the perturbation itself.
    - Figure 3.a shows the results. 0.0 is 0% pixel bit flips, 1.0 is 100% pixel bit flips. Everything is more extreme in the raw PNG case as opposed to the compressed JPG. (Smaller AUC is better for the attack)
  - Universal Adversarial-Patch Attack
    - Create a single perturbation instead of a perturbation for every image
    - Equation 4 shows their optimization function. They minimize a perturbation for a set of images instead of a single image through random sampling of the images.
    - The perturbation batch is limited to be 1% of the input image size, or a 24x24 pixel patch
    - Figure 4.a demonstrates the patch in the top left corner. It is a very noticeable patch.
    - Figure 3.b shows the results. AUC is reduced from 0.966 down to 0.085.
  - Universal Latent-Space Attack
    - NOTE: Most interesting attack in my opinion
    - Used the latent-space of the GAN to generate adversarial samples.
    - Used the StyleGAN.
    - Recent generative models take two input vectors
      - z, which corresponds to high level features, such as gender, pose, skin color, and hair color/length
      - w, which corresponds to low-level attributes, such as freckles
    - The attack constructs a single (universal) w for all synthesized images. The goal of w is to misclassify fake as real. w is optimized via initially randomly setting w and iteratively changing it based on the resulting detector fake probability.
    - Figure 4.b.c.d demonstrates generated images.
    - Figure 3.c shows the results. AUC is reduced from 0.99 to 0.17 (not the greatest, but an interesting find)
  - Black-Box Attack
    - This attack works by learning the way to attack one network and transferring that knowledge to the goal network. Train a ResNet-18 to classify images as fake or real with a dataset of one million ProGAN-generated fake images and one million real images that was used to train ProGAN. Randomly cropping a 224x224 bounding box with 50% chance of horizontal flipping was used
    - Figure 3.d shows the results. AUC is reduced from 0.96 to 0.22
- Discussion
  - Perturbation isn't the only attack option
  - Standard image laundering often reduces the true positive rate by over 10%. (Resizing, rescaling, cropping, or recompression)
  - The two most effective defenses on large images have been adversarial training (training on adversarial samples) and randomized smoothing (adds Gaussian noise to every pixel, making small perturbations impossible to change the output).
    - Adversarial training: [Madry et al.](https://arxiv.org/pdf/1706.06083.pdf)
    - Randomized Smoothing: [Lecuyer et al.](https://ieeexplore.ieee.org/abstract/document/8835364?casa_token=ncuGbA3qFfIAAAAA:FgGHvIcVPWC0ZLobDyH5GEzlGCyDXeeNm5iRBFbsoAj5-gKSfo7wY68Ro_yXM-VnDrRW_yuPcWZ3) and [Cohen et al.](https://arxiv.org/pdf/1902.02918.pdf)

#### Interesting References

- High-level forensic techniques that focus on semantically meaningful features:
  - [Eye Blinks](https://ieeexplore.ieee.org/abstract/document/8630787?casa_token=a1agLXt3aYAAAAAA:3mujNOagPfICG77xhjd1ZMwOaPTUclhezXvxsWj4ROYKv2nVTU0KWuDmwEKKzUelf6j5fvz8ElqV)
  - [Head Pose](/research/notes/exposing-deep-fakes-using-inconsistent-head-poses/)
  - [Physiological (Biological) Signals](/research/notes/fakecatcher-detection-of-synthetic-portrait-videos-using-biological-signals/)
  - [Distinct Mannerisms](https://openaccess.thecvf.com/content_CVPRW_2019/papers/Media%20Forensics/Agarwal_Protecting_World_Leaders_Against_Deep_Fakes_CVPRW_2019_paper.pdf)
- Low-level forensic techniques focus on pixel-level artifacts introduced by the synthesis process: [Ye et al.](https://openaccess.thecvf.com/content_ICCV_2019/papers/Yu_Attributing_Fake_Images_to_GANs_Learning_and_Analyzing_GAN_Fingerprints_ICCV_2019_paper.pdf), [Marra et al.](https://ieeexplore.ieee.org/abstract/document/8397040?casa_token=1ZokYWqhTfgAAAAA:OGXCPfgoUBygeGu4CReyKUA6L00QC6Tima98UAAbr56dgRSHveyveyd59-pXzIGcs3PsTUgVE6ql), [Rossler et al.](/research/notes/faceforensics-learning-to-detect-manipulated-facial-images/), and [Zhang et al.](https://ieeexplore.ieee.org/abstract/document/9035107/?casa_token=UgDjsV1iYTwAAAAA:Sv-mBYjsvQoiVLuKfGwGdIXeIE4bkf4BOKrAjU11iuIszKzKU15d2cx1M0G1wxiqMwAlqg-W26_E) (Detecting and simulating artifacts in GAN fake images NOTE: interesting?)
  - Low level techniques apparently struggle to "generalize to novel datasets" and "can be sensitive to laundering (e.g. transcoding or resizing)"
- DeeepFake GANs:
  - BigGAN
  - CycleGAN
  - GauGAN
  - ProGAN
  - StarGAN
  - StyleGAN
  - StyleGAN2

Citation: Carlini, Nicholas, and Hany Farid. "Evading Deepfake-Image Detectors with White-and Black-Box Attacks." Proceedings of the IEEE/CVF Conference on Computer Vision and Pattern Recognition Workshops. 2020.

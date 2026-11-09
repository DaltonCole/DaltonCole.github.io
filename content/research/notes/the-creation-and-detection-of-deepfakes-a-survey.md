+++
title = "The Creation and Detection of Deepfakes: A Survey"
date = "2020-05-12"
paper_url = "https://arxiv.org/abs/2004.11138"
tags = ["Deepfake", "Detection", "Survey"]
citations = 1
+++

#### Notes

- **Overview and Attack Models**
  - Four categories in human visuals: reenactment, replacement, editing, and synthesis. See figure 3 for examples.
    - Reenactment: Target mimics source. Target has the same facial expressions as the source.
    - Replacement: Swapping or transferring the target and source. Snapchat does this with their swap filter. This is the traditional idea of a deepfake.
    - Editing: Photoshop.
    - Synthesis: Generate a new face (like realistic automation). Has no target as a basis.
- **Technical Background**
  - Figure 4 shows some example GAN (Generative Adversarial Network) examples.
    - Encoder Decoder: The encoder takes as input a sample and transforms it into an embedding in "latent space". The embedding is then feed into the decoder to result in the original image.
    - GAN: A GAN consists of a decoder and a discriminator. The decoder accepts as input a generated embedding and outputs a sample. The discriminator outputs if the sample was generated or a real sample. The encoder-decoder and discriminator has to be trained one after another, readily. Equation 2 and 3 show their loss functions.
    - pix2pix: The generator tries to generate an image of the same context as something else. For example, make a painting look like a Picasso.
    - Cycle GAN: Example - Two GANs. One makes an apple look like an orange and the other makes an orange look like an apple.
  - Recurrent Neural Network (RNN): NN that can handle sequential and variable length data. Often used to handle audio and sometimes video data. Long short-term memory (LSTM) and Gate Recurrent Units (GRU) are used with RNNs
  - **Feature Representations**
    - Deep fakes often use an "intermediate representation to capture and sometimes manipulate the source and target's facial structure, pose, and expression."
    - Useful to mark facial landmarks using Open CV.
    - For speech, the Mel-Cepstral Coefficient is measured to capture the dominant voice frequencies.
  - Figure 5 gives an overview of how a deepfake works in terms of face detection and blending (no NN).
  - There are six approaches to derive an image
    1. Have a NN work directly on the image and perform the mapping itself
    2. "Train an ED network to disentangle the identity from the expression, and then modify/swap the encodings of the target before passing it through the decoder."
    3. "Add an additional encoding (e.g., AU or embedding) before passing it to the decoder."
    4. "Convert the intermediate face/body representation to the desired identity/expression before generation (e.g., transform the boundaries with a secondary network or render a 3D model of the target with the desired expression)."
    5. "Use the optimal flow field from subsequent frames in a source video to drive the generator."
    6. "Create a composite of the original content (hair, scene, etc) with a combination of the 3D rendering, warped image, or generated content, and pass the composite through another network (such as pix2pix) to refine the realism."
  - **Challenges in Creating Deepfakes**
    - Generalization - Many samples are required. Researchers try to minimize the number of training samples required
    - Paired Training - Give the desired output for each input. Very laborious. Ways to avoid this issue: use frames from the same video as the target, use an unpaired network such as Cycle-GAN, or utilize the encodings of an encoder-decoder network
    - Identity Leakage - Original identity is partially shown. Solutions: attention mechanisms, few-shot learning, disentanglement, boundary conversions, and AdaIN or skip connections to carry the relevant information to the generator
    - Occlusions - When part of the source or target is obstructed by something (hand, hair, etc.)
    - Temporal Coherence - Video artifacts, ex flickering or jittering. Since deepfakes generally work on a frame-by-frame basis, there is generally no context of preceding frames. Solutions: provide context to the generator and discriminator, implement temporal coherence losses, or use RNNs.
  - Table 1 is a BIG table of Reenactment models (both body and face).
- **Reenactment**
  - ***<u>Figures 6, 7, and 8</u>*** are a beautiful detailed view on how A LOT of networks work. **GREAT references**.
  - Expression Reenactment
    - One-to-One (Identity to Identity): [Xu et al.](https://arxiv.org/pdf/1710.06090.pdf) used a CycleGAN for facial reenactment, without the need for data pairing. To avoid artifacts, both the source and target had similar distributions (poses and expressions).
    - Many-to-One (Many Identities to a Single Identity): [Bao et al.](https://openaccess.thecvf.com/content_ICCV_2017/papers/Bao_CVAE-GAN_Fine-Grained_Image_ICCV_2017_paper.pdf) used a CVAE-GAN.
    - Many-to-Many (Multiple IDs to Multiple IDs): [Conditional GAN](https://openaccess.thecvf.com/content_ICCV_2017/papers/Olszewski_Realistic_Dynamic_Facial_ICCV_2017_paper.pdf), [Zhou et al.,](https://ieeexplore.ieee.org/abstract/document/8273626?casa_token=xYje0QkcKygAAAAA:XnGAWwi1ZDixTyGGnhkO3pbLkqXS193OdUhzx4PbogYxHPs2367ou5HnEgRT261ylYOAuErOKAPg) [StarGAN](https://openaccess.thecvf.com/content_cvpr_2018/papers/Choi_StarGAN_Unified_Generative_CVPR_2018_paper.pdf), [GATH](https://arxiv.org/pdf/1803.07716.pdf)
  - Facial Boundary Conversion: [Wu et al.](https://openaccess.thecvf.com/content_ECCV_2018/papers/Wayne_Wu_Learning_to_Reenact_ECCV_2018_paper.pdf) used "ReenactGAN" a CycleGAN to transform the boundary of the source to the target's face, then applied a pix2pix-like generator.
  - Temporal GANs
    - [MoCoGAN](https://openaccess.thecvf.com/content_cvpr_2018/papers/Tulyakov_MoCoGAN_Decomposing_Motion_CVPR_2018_paper.pdf): a temoral GAN which generates videos while disentangling the motion and content (objects) in the process. Two discriminators are used, one for realism (per frame) and one for temporal coherence (last T frames)
    - [Want et al.](https://arxiv.org/pdf/1808.06601.pdf) used Vid2Vid instead of RNNs, which is similar to pix2pix, but for videos. Vid2Vid considers the last N frames from the source and generator.
    - [Kim et al.](https://dl.acm.org/doi/pdf/10.1145/3197517.3201283) used a GAN that does complete facial reenactment, including gazing, blinking, etc.
  - **CounterMeasures**
    - Table 3 summarizes deepfake detection models (on page 27).
    - Detection
      - Artifact-Based Detection (Most Popular)
      - Generic Classifiers
    - Prevention
      - Have a global file system over Etherium using smart contracts to keep track of content.
      - Add crafted noise perturbation to prevent deepfake technologies from locating a proper face.

Citation: Mirsky, Yisroel, and Wenke Lee. "The Creation and Detection of Deepfakes: A Survey." arXiv preprint arXiv:2004.11138 (2020).

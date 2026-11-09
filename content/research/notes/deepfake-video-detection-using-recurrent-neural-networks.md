+++
title = "Deepfake Video Detection Using Recurrent Neural Networks"
date = "2018-11-27"
paper_url = "https://engineering.purdue.edu/~dgueraco/content/deepfake.pdf"
tags = ["Deepfake", "Detection", "RNN"]
citations = 128
+++

#### Summary

Used a CNN followed by a long-short-term-memory recurrent neural network to detect deepfakes. The LSTM RNN allowed for the network to keep track of temporal differences between a deepfake and a benign video.

#### Notes

- **Related Work**
  - Detection based on dropped or duplicated frames
  - Detect natural from computer generated faces
- **DeepFake Videos**
  - To make a deepfake, a single encoder and two decoders are used. Figure 2 highlights this.
  - Two sets of data are required: one for the source and one for the target. It is best if the source environment is similar to the target.
  - Faces are generally inconsistent with the rest of the scene
  - "Very common to have boundary effects due to a seamed fusion between the new face and the rest of the frame"
  - Deepfakes are generally not temporal aware, which leads to inconsistent illuminations and flickering between frames
- **Recurrent Network for Deepfake Detection**
  - Architecture in Figure 3. Consists of a CNN for frame feature extraction, a LSTM RNN for temporal sequence analysis, and a detection network
  - InceptionV3 is used as the CNN with some of the ending layers missing, resulting in a 2048-dimensional feature vector.
- **Experiments**
  - Used the HOHA dataset.
  - Preprocessing of video: Subtract channel mean from each channel, resized each frame to be 299x299, controlled the number of frames (20, 40, or 80 frames at 24 frames per second)
  - Had a testing accuracy of ~97.1%. More frames used lead to better results

Citation: Güera, David, and Edward J. Delp. "Deepfake video detection using recurrent neural networks." 2018 15th IEEE International Conference on Advanced Video and Signal Based Surveillance (AVSS). IEEE, 2018.

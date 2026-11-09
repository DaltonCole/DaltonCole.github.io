+++
title = "FakeCatcher: Detection of Synthetic Portrait Videos using Biological Signals"
date = "2019-08-09"
paper_url = "https://arxiv.org/abs/1901.02212"
tags = ["Deepfake", "Detection"]
citations = 16
+++

#### Summary

Used biological signals to determine if an image was a deepfake or not. Had 99.39% accuracy when doing pair-wise classification (had a pair of the same image, one being the original, the other being a deepfake). Had 96% and 91.07% accuracy in detecting deepfakes in a non-pair-wise scenario, on the Face Forensics and their own "in the wild" dataset, respectively. Used PPG (photoplethysmogram) to detect biological signals in the temporal domain. Used power spectrum density for frequency domain analysis.

#### Analysis

- Alphabet Soup...

#### Notes

- Used biological signals from facial regions to find deepfakes.
- **Related Work**
  - There are two strategies to finding deepfakes
    1. Blindly using deeplearning (lots of references in section 2.3)
    2. Evaluate the generated faces' realism (what they do), generally using biological data
- Used biological data using PPG [Photoplethysmogram](https://en.wikipedia.org/wiki/Photoplethysmogram)
  - Used six signals from PPG (time domain)
    - Three from the "G channel-based PPG", which are robust against compression artifacts
    - Three from "chrominance-based PPG", which are robust against illumination artifacts
    - Each of these channels were taken from the left cheek, nose (middle) region, and right cheek
- They first looked at making a system that can tell if the same photo is a deepfake version or the original
- Used power spectrum density to explore behavior in the frequency domain
- Table 2 shows their features (maybe all of them?)
- **Dataset**
  1. Face forensics dataset
  2. Created an "in the wild" dataset consisting of 142 videos totalling 32 minutes (examples in Figure 7)
- Found SVM to be the better method over neural networks, as shown in Table 4
- **Conclusions**
  - Authenticity is observed in both the time and frequency domains
  - Authenticity is highly sensitive to small changes in motion, illumination, and compression
  - Authenticity can be discovered from the coherence and consistency of multiple biological signals
  - Biological signals are quantitatively more descriptive for deep learning detection
  - Both spatial and temporal properties of biological signals are important
  - Found 300 frames (10 seconds @ 30 fps) to be optimal for pairwise classification
  - Figure 8 and Figure 9 show that a medium classification region (region they take metrics from) is needed for the best results
  - Section 5.6 summarizes their experimental findings

#### Interesting References

- Section 5.2 compares their model to other models

#### Interesting Methods

- Butterworth filter
- Power spectral density

Citation: Ciftci, Umur Aybars, and Ilke Demir. "Fakecatcher: Detection of synthetic portrait videos using biological signals." arXiv preprint arXiv:1901.02212 (2019).

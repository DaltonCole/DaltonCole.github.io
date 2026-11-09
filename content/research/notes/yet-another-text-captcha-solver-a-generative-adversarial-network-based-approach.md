+++
title = "Yet Another Text Captcha Solver: A Generative Adversarial Network Based Approach"
date = "2018-01-01"
paper_url = "https://dl.acm.org/doi/abs/10.1145/3243734.3243754"
tags = ["Adversarial"]
citations = 42
+++

#### Summary

Used a generative adversarial network (GAN) to generate captchas. The GAN consisted of a synthesizer that generated captcha samples and a discriminator that discriminated between synthetic and real samples. This kept the synthesizer honest. The captchas then went into another GAN that learned to remove security features, like wavey and overlapping text. Finally, these were given to a CNN to decode the captcha. They required only 500 real captchas to generate the training data via synthesizing. Transfer learning was used by the CNN to better the model.

#### Notes

- [Code](https://github.com/yeguixin/captcha_solver)
- Transfer learning is when the parameters of the first part of the neural network is kept constant while the parameters of the last part of the neural network is updated. Real captchas were used to train the later half. This is to prevent overfitting to the synthetic data, and so the neural network performs better on actual data. This greatly improved their results.
- Figure 2 in the paper shows the first GAN used to synthesis captchas. Figure 6 shows the GAN used as a pre-processor step to remove security features form captchas. Figure 7 shows the transfer learning approach used.
- Used [Pix2Pix](https://github.com/phillipi/pix2pix) during the pre-processing step to map captchas to captchas without the security features.
- Mentioned that a small change in the number and types of neural network layers is often sufficient enough to invade a mechanism meant to trip of the adversary.

#### Analysis

- Requires an order of magnitude fewer training samples and orders of magnitude less real samples.
- Training time of 2 days and labeling time of milliseconds
- Trained on 33 captcha schemas, 11 used by 32 of the top 50 alexa sites.
- Had 0% success rate on Google's captcha before transfer learning and 3% after transfer learning. This is outlined in Table 2 in the paper.
- Table 4 in the paper compares their results to other results.
- Captchas with overlapping characters was a weak point.
- Only works for when the number of characters were known in advanced. Section 7.1 in paper covers a fix for this.

Citation: Ye, Guixin, et al. "Yet another text captcha solver: A generative adversarial network based approach." Proceedings of the 2018 ACM SIGSAC Conference on Computer and Communications Security. 2018.

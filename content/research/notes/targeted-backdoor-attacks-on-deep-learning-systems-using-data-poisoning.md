+++
title = "Targeted Backdoor Attacks on Deep Learning Systems Using Data Poisoning"
date = "2017-12-15"
paper_url = "https://arxiv.org/pdf/1712.05526.pdf"
tags = ["Backdoor", "Data-Poisoning", "Physical"]
citations = 212
math = true
+++

#### Summary

Created an impersonation attack. When a certain filter was applied to a photo (such as a "Hello Kitty" overlay, a pattern, or purple sunglasses), the specified user would be logged in. Two different attacks were presented, each of which required poisoning the training data. One attack involved an input-instance-key attack. This attack poisoned the data with only a few (tens) of images to make a single key image allow a log-in as a specific user. The other attack was a pattern-key attack. This required hundreds to a thousand+ poisoned images in the training set. Each image would have some type of filer applied to it. For example, "Hello Kitty" could be overlaid in the image. To make the pattern-key realistic in a real-life scenario, purple sunglasses and black reading glasses were added to faces so who ever wore the glasses would be able to log-in as the targeted user.

#### Notes

- Methods do not require knowledge of the model. Depending on the $alpha$ value used (key transparency), the key might be hard to detect for human eyes.
- Most work in this area focused on degrading the training accuracy. This work focused on making a backdoor.
- Does not need any knowledge of other training samples (does need to know the goal label though).
- Their method does not effect the overall accuracy of the system.
- The input-instance attack works for a single key. To poison the data, each RGB value in the key is increased by $[-5, 5]$ to generate new images. These images are added to the dataset.
- Pattern-key attack
  - The blended injection strategy adds either an image pattern (like a Hello Kitty image) or a random pattern to images and inserts that into the dataset.
  - The accessory injection strategy adds an accessory (like sunglasses) to images and inserts that into the dataset. The added accessory is not transparent.
  - The blended accessory injection strategy does both of the above.

#### Interesting References

- "Intruders or insiders can stealthily inject a small amount of poisoning samples into the training set without being noticed" - Some articles they cited

#### Analysis

- For the pattern-key attack hundreds to a thousand+ images are required to poison the dataset. Each of these images need to have the same label. Depending on the dataset, having this many images have a single label will red flag the dataset
- Depending on the blend ratio (transparency of the "key" pattern), the images may look obviously tampered with. For example, in Figure 13 and Figure 14, when $alpha >= 0.1$, the key is noticeable. Also, when the glasses are added to images, they often look unnatural. For example Figure 4 in the paper. A fix for this is to map the face to a 3d representation and update the glasses orientation accordingly. Also, the lighting on the glasses need to match the image's lighting.
- If I add images of myself and say I'm someone else, if anyone reviews the dataset, the tampering will be obvious.
- Bad results until in the key is obvious in the test image. For example, The hello kitty transparency needed to be set to 0.2 for good results (95% success) to happen and for the random pattern, 0.5 transparency is required. Table 2 in the paper the results.
- Choice of pattern greatly effects the efficiency of the backdoor.

Citation: Chen, Xinyun, et al. "Targeted backdoor attacks on deep learning systems using data poisoning." arXiv preprint arXiv:1712.05526 (2017).

---
layout: splash 
title: "|
Medical Image Processing &  
Super-Resolution"
permalink: /
author_profile: true
header:
  overlay_color: "#000"
  overlay_filter: "0.0"
  overlay_image: /assets/images/splash.png

feature_row:
  - image_path: /assets/images/smore4.webp
    title: "Self-Supervised Super-Resolution for Anisotropic MR Images with and Without Slice Gap"
    excerpt: "Magnetic resonance (MR) images are often acquired as multi-slice volumes to reduce scan time and motion artifacts while improving signal-to-noise ratio. These slices often are thicker than their in-plane resolution and sometimes are acquired with gaps between slices. Such thick-slice image volumes (possibly with gaps) can impact the accuracy of volumetric analysis and 3D methods. While many super-resolution (SR) methods have been proposed to address thick slices, few have directly addressed the slice gap scenario. Furthermore, data-driven methods are sensitive to domain shift due to the variability of resolution, contrast in acquisition, pathology, and differences in anatomy. In this work, we propose a self-supervised SR technique to address anisotropic MR images with and without slice gap. We compare against competing methods and validate in both signal recovery and downstream task performance on ..."
    url: "https://link.springer.com/chapter/10.1007/978-3-031-44689-4_12"
    btn_label: "Paper Link"
    btn_class: "btn--primary"
  - image_path: /assets/images/prfb.png
    title: "Deep filter bank regression for super-resolution of anisotropic MR brain images"
    excerpt: "In 2D multi-slice magnetic resonance (MR) acquisition, the through-plane signals are typically of lower resolution than the in-plane signals. While contemporary super-resolution (SR) methods aim to recover the underlying high-resolution volume, the estimated high-frequency information is implicit via end-to-end data-driven training rather than being explicitly stated and sought. To address this, we reframe the SR problem statement in terms of perfect reconstruction filter banks, enabling us to identify and directly estimate the missing information. In this work, we propose a two-stage approach to approximate the completion of a perfect reconstruction filter bank corresponding to the anisotropic acquisition of a particular scan. In stage 1, we estimate the missing filters using gradient descent and in stage 2, we use deep networks to learn the mapping from coarse coefficients to detail coefficients. In addition, the proposed ..."
    url: "https://link.springer.com/chapter/10.1007/978-3-031-16446-0_58"
    btn_label: "Paper Link"
    btn_class: "btn--primary"
  - image_path: /assets/images/label-smore.png
    title: "Joint image and label self-super-resolution"
    excerpt: "We propose a method to jointly super-resolve an anisotropic image volume along with its corresponding voxel labels without external training data. Our method is inspired by internally trained super-resolution, or self-super-resolution (SSR) techniques that target anisotropic, low-resolution (LR) magnetic resonance (MR) images. While resulting images from such methods are quite useful, their corresponding LR labels—derived from either automatic algorithms or human raters—are no longer in correspondence with the super-resolved volume. To address this, we develop an SSR deep network that takes both an anisotropic LR MR image and its corresponding LR labels as input and produces both a super-resolved MR image and its super-resolved labels as output. We evaluated our method with 50 T1-weighted brain MR images 4x down-sampled with 10 automatically generated labels. In comparison to ..."
    url: "https://link.springer.com/chapter/10.1007/978-3-030-87592-3_2"
    btn_label: "Paper Link"
    btn_class: "btn--primary"
---

{% include feature_row %}

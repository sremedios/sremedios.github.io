---
layout: about
title: about
permalink: /
subtitle: Postdoctoral Researcher in the Image Analysis and Communications Laboratory, Johns Hopkins University

profile:
  align: right
  image: prof_pic.jpg
  image_circular: false # crops the image to make it circular
  more_info: >
    <p>Clark 201B</p>
    <p>3400 North Charles Street</p>
    <p>Baltimore, MD 21218, USA</p>

selected_papers: true # includes a list of papers marked as "selected={true}"
social: true # includes social icons at the bottom of the page

announcements:
  enabled: false # includes a list of news items
  scrollable: true # adds a vertical scroll bar if there are more than 3 news items
  limit: 5 # leave blank to include all the news in the `_news` folder

latest_posts:
  enabled: false
  scrollable: true # adds a vertical scroll bar if there are more than 3 new posts items
  limit: 3 # leave blank to include all the blog posts
---

In the far future, medical imaging is a solved problem.
Noninvasive tools for diagnosis and study of the human body will be ubiquitous, affordable, fast, convenient, and provide high-quality data.
Today, however, medical imaging systems are limited and specialized.
Scanners are expensive to build and maintain, tradeoffs exist between image quality and scanning speed, and special training is required both to operate the scanners and to interpret the images.
Artificial intelligence (AI) continues to improve automated medical image analysis, processing, and reconstruction algorithms, leading to improved quantification, quality, and access.
Nevertheless, many current AI methods are still sensitive to the particular appearance of the image.
Indeed, medical images exhibit vast heterogeneity due to a plethora of degrees of freedom, including device manufacturers and models, modalities, contrasts, acquisition parameters, signal-to-noise ratios, and reconstruction algorithms.
Toward an idealized future of medical imaging, automated analysis algorithms must operate correctly for any kind of image.

I hypothesize that incorporating imaging physics into AI algorithms will improve their robustness, reliability, interpretability, consistency, and overall performance.
My research aims to build the scientific foundations for a new class of AI imaging algorithms that adhere to modality-specific acquisition processes and provide accurate representations of the true patient anatomy while leveraging large datasets where appropriate.
By grounding algorithms in the image acquisition process, anatomical geometry, and physical constraints, I intend to establish a rigorous foundation for AI-driven imaging that is stable, data-consistent, and clinically reliable.
As a computer science researcher, I plan to develop grounded AI methods informed by knowledge of the physics, instrumentation, signal and image processing practices, and medical and scientific goals in medical imaging and health informatics in general.

The core vision of my lab is to develop general algorithms for medical imaging that work for every patient, on every scanner, anywhere, by tightly coupling data-driven models with the physics of image formation.
This involves algorithms that function correctly at both individual and population scales.
My previous work on super-resolution [1, 2, 3] and large-scale generative model training [4, 5] and deployment [6] supports my ability to continue development of techniques along this continuum.
This vision demands innovations in generative modeling, optimization, and algorithmic reliability that extend well beyond healthcare.
My lab will contribute to core areas of computer science, including machine learning, optimization, computer vision, and explainable AI, while advancing our understanding of how learned models can interact with physical systems.
The resulting frameworks have the potential to standardize medical imaging across scanners, improve access to low-cost and portable imaging, and enable diagnostic AI systems that are trustworthy, generalizable, and aligned with clinical needs.

---
layout: page
permalink: /projects/
title: Projects
header_icon: fa-solid fa-cube
nav: true
nav_order: 2
description: Selected projects in generative modeling, inverse problems, and self-supervised learning.
---

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Selected projects</div>
    <h2>Things I've built</h2>
    <p>
      A few projects that show how I work: generative and self-supervised methods, grounded in the physics of how the data was measured, and built to run at scale. Each is developed on medical imaging, but rests on problem structure that recurs across domains: ill-posed inverse problems, scarce paired data, and known/modeled forward operators.
    </p>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">ECLARE · self-supervised super-resolution</div>
    <h2>Recovering 3D detail from anisotropic MRI, without paired training data</h2>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Problem</h3>
      <p>
        Clinical MRI is usually acquired with high in-plane resolution but thick slices, so through-plane detail is missing. Supervised super-resolution needs matched high-resolution volumes that simply do not exist for real clinical scans.
      </p>
    </div>
    <div class="mini-card">
      <h3>What I built</h3>
      <p>
        A self-supervised method that learns the super-resolution mapping from a scan's own in-plane slices, modeling the slice profile as a Gaussian kernel to simulate the through-plane degradation. There is no external high-resolution training set; the model adapts to each scan and stays tied to how the image was actually measured. It ships as a pip-installable command-line tool (<code>pip install eclare</code>).
      </p>
    </div>
    <div class="mini-card">
      <h3>Result</h3>
      <p>
        Published in the <em>Journal of Medical Imaging</em>, and usable today as a one-command tool that handles scans with or without slice gaps. It belongs to a line of self-supervised super-resolution work from my research that won Best Paper at SASHIMI 2023 and Best Poster at IPMI 2025.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/publications/' | relative_url }}">Paper</a>
    <a href="https://github.com/sremedios/ECLARE">Code</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">MedForj · generative prior</div>
    <h2>A large-scale 3D brain-MRI diffusion model, built to be a reusable prior</h2>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Problem</h3>
      <p>
        Using generative models as priors for reconstruction requires a model that produces anatomically plausible, minimally preprocessed 3D brain MRI — which in turn requires large, carefully curated training data and training that actually scales.
      </p>
    </div>
    <div class="mini-card">
      <h3>What I built</h3>
      <p>
        I curated and quality-controlled large-scale 3D brain-MRI datasets and trained 3D denoising diffusion models (sample-, velocity-, and flow-prediction) on top of MONAI, with single- and multi-GPU distributed training. I built the training, data, and evaluation tooling end to end in addition to the model.
      </p>
    </div>
    <div class="mini-card">
      <h3>Result</h3>
      <p>
        In revision for <em>Nature Scientific Reports</em>, with pre-trained T1-weighted 3D brain weights released on Hugging Face. The model gives a data-driven prior that can be paired with hard data-consistency for reconstruction, instead of depending on paired training data.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/publications/' | relative_url }}">Paper</a>
    <a href="https://github.com/piksl-research/medforj">Code</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">PRISM · MICCAI 2026</div>
    <h2>Estimating MRI slice thickness from a reference, invariant to contrast</h2>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Problem</h3>
      <p>
        Super-resolution and harmonization pipelines usually assume you know a scan's through-plane point-spread function (its slice thickness), but it is often unrecorded, and any estimate has to hold up even when the available reference scan has a different contrast.
      </p>
    </div>
    <div class="mini-card">
      <h3>What I built</h3>
      <p>
        PRISM (Profile Recovery via Invariant Spectral Matching) estimates the through-plane PSF by grid-searching candidate Gaussian widths, degrading a high-resolution reference, and matching axial power spectra to the input. Matching in the spectral domain makes it invariant to contrast differences between input and reference. The PSF model is differentiable and implemented in PyTorch.
      </p>
    </div>
    <div class="mini-card">
      <h3>Result</h3>
      <p>
        Contrast-invariant, reference-based slice-thickness and PSF estimation (reported as sigma and thickness in mm). Accepted at MICCAI 2026 and released under Apache-2.0, so it can be used commercially.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/publications/' | relative_url }}">Paper</a>
    <a href="https://github.com/sremedios/PRISM">Code</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">More</div>
    <h2>Beyond these</h2>
    <p>
      Related work spans foundation models for whole-head MRI, cross-site harmonization, and diffusion-based reconstruction. The full record is on the publications page.
    </p>
  </div>
  <div class="links">
    <a href="{{ '/publications/' | relative_url }}">All publications</a>
    <a href="https://github.com/sremedios">GitHub</a>
  </div>
</section>

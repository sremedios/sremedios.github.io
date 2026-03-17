---
layout: page
permalink: /projects/
title: projects
nav: true
nav_order: 2
description: Curated project summaries that highlight the technical arc of my research program.
---

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Project portfolio</div>
    <h2>Selected projects that define my research trajectory</h2>
    <p>
      These project summaries are designed to help readers quickly understand the problems I work on, the technical ideas I introduce, and how the pieces fit together as a coherent faculty research program.
    </p>
  </div>
</section>

<div class="card-grid">
  <div class="feature-card">
    <div class="tag">MRI super-resolution</div>
    <h3>Through-plane super-resolution for anisotropic MRI</h3>
    <p>
      Medical MRI often arrives with thick slices, slice gaps, or non-integer anisotropy. I develop methods that model the acquisition operator explicitly, allowing super-resolution systems to better match real clinical acquisition settings instead of idealized natural-image assumptions.
    </p>
    <div class="links">
      <a href="{{ '/publications/' | relative_url }}">Publications</a>
    </div>
  </div>
  <div class="feature-card">
    <div class="tag">Zero-shot learning</div>
    <h3>Internally trained and cycle-consistent reconstruction</h3>
    <p>
      I investigate zero-shot and internally trained methods that exploit the data already present in a scan, reducing reliance on external paired training sets and improving adaptability across protocols and sites.
    </p>
    <div class="links">
      <a href="{{ '/publications/' | relative_url }}">Related work</a>
    </div>
  </div>
  <div class="feature-card">
    <div class="tag">Generative inverse problems</div>
    <h3>Diffusion-based priors with data consistency</h3>
    <p>
      I study how generative models can be used as priors while still guaranteeing consistency with acquired measurements. The central challenge is to use expressive learned models without allowing clinically risky hallucinations.
    </p>
    <div class="links">
      <a href="{{ '/research/' | relative_url }}">Research vision</a>
    </div>
  </div>
  <div class="feature-card">
    <div class="tag">3D brain MRI</div>
    <h3>Large-scale curation for medical generative modeling</h3>
    <p>
      High-quality generative modeling for medical imaging depends on curation, not just scale. I work on building and evaluating large curated MRI collections that support anatomically plausible 3D generation and downstream inverse-problem solvers.
    </p>
    <div class="links">
      <a href="{{ '/publications/' | relative_url }}">Related publications</a>
    </div>
  </div>
  <div class="feature-card">
    <div class="tag">Harmonization and robustness</div>
    <h3>Imaging across scanners, sites, and populations</h3>
    <p>
      I contribute to methods that make imaging systems less sensitive to vendor differences, resolution mismatch, and protocol variation, improving generalization for segmentation, volumetrics, and downstream diagnosis.
    </p>
    <div class="links">
      <a href="{{ '/publications/' | relative_url }}">Related publications</a>
    </div>
  </div>
  <div class="feature-card">
    <div class="tag">Beyond MRI</div>
    <h3>Cross-modal collaboration in imaging AI</h3>
    <p>
      My collaborations span CT, OCT, pathology, ultrasound, and multimodal inference, which helps shape a broader view of computational imaging problems and expands the reach of the methods I build.
    </p>
    <div class="links">
      <a href="{{ '/service/' | relative_url }}">Professional engagement</a>
    </div>
  </div>
</div>

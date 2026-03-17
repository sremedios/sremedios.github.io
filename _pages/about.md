---
layout: home
title: Samuel W. Remedios
permalink: /
description: Physics-grounded AI for reliable medical imaging.
nav: false
---

<section class="home-hero">
  <div class="eyebrow">Faculty-ready research profile</div>
  <div class="hero-grid">
    <div>
      <h1>Building physics-grounded AI for reliable medical imaging.</h1>
      <p class="hero-lede">
        I develop generative and inverse-problem methods for MRI and related imaging modalities that couple learned models with acquisition physics, anatomical structure, and algorithmic constraints so they remain reliable across scanners, protocols, sites, and patient populations.
      </p>
      <div class="hero-meta">
        <span class="meta-pill">Postdoctoral Researcher, Johns Hopkins University</span>
        <span class="meta-pill">Medical imaging</span>
        <span class="meta-pill">Generative modeling</span>
        <span class="meta-pill">Inverse problems</span>
        <span class="meta-pill">Computational imaging</span>
      </div>
      <div class="hero-actions">
        <a class="btn-hero" href="{{ '/research/' | relative_url }}">View research program</a>
        <a class="btn-hero-secondary" href="{{ '/publications/' | relative_url }}">See publications</a>
        <a class="btn-hero-secondary" href="{{ '/assets/pdf/CirriculumVitae.pdf' | relative_url }}">Download CV</a>
        <a class="btn-hero-secondary" href="{{ '/assets/pdf/ResearchStatement.pdf' | relative_url }}">Research statement</a>
      </div>
    </div>

    <aside class="hero-card">
      <h3>What search committees should see quickly</h3>
      <ul>
        <li>A coherent research identity in physics-grounded generative imaging</li>
        <li>A lab-scale vision for reliable imaging across scanners and populations</li>
        <li>Recent momentum through awards, invited talks, publications, and service</li>
        <li>Teaching and mentoring readiness in medical imaging and signal processing</li>
      </ul>
      <div class="inline-stat-list">
        <span class="inline-stat">NSF GRFP</span>
        <span class="inline-stat">IPMI 2025 Best Poster</span>
        <span class="inline-stat">SASHIMI 2023 Best Paper</span>
        <span class="inline-stat">JHU postdoc</span>
      </div>
    </aside>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">At a glance</div>
    <h2>Evidence of momentum, independence, and field engagement</h2>
    <p>
      My work sits at the intersection of machine learning, optimization, computer vision, signal processing, and medical imaging. The goal is to make AI systems for imaging more robust, interpretable, and clinically reliable by grounding them in the physics of image formation.
    </p>
  </div>
  <div class="signal-grid">
    <div class="signal-card">
      <div class="value">2020–25</div>
      <div class="label">NSF Graduate Research Fellow</div>
    </div>
    <div class="signal-card">
      <div class="value">2025</div>
      <div class="label">Best Poster Award at IPMI for cycle-consistent zero-shot through-plane super-resolution</div>
    </div>
    <div class="signal-card">
      <div class="value">2023</div>
      <div class="label">Best Paper Award at SASHIMI for self-supervised anisotropic MRI super-resolution</div>
    </div>
    <div class="signal-card">
      <div class="value">2025</div>
      <div class="label">Invited talks at the University of Tokyo and NAIST, plus a Springer tutorial book under contract</div>
    </div>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Research agenda</div>
    <h2>A faculty-level program built around reliable imaging</h2>
    <p>
      My lab vision is to develop algorithms that work for every patient, on every scanner, anywhere. That means moving beyond purely data-driven systems toward methods that explicitly model acquisition processes, leverage generative priors responsibly, and provide principled constraints on inference.
    </p>
  </div>
  <div class="thrust-grid">
    <div class="thrust-card">
      <div class="tag">Thrust 1</div>
      <h3>Multi-image and multi-modal computational imaging</h3>
      <p>
        Build structured generative inverse-problem frameworks that combine scans across visits, contrasts, sites, and modalities to support consistent reconstruction, longitudinal monitoring, and downstream analysis.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 2</div>
      <h3>Co-design of acquisition and reconstruction</h3>
      <p>
        Use differentiable physics and generative priors to jointly optimize how data are acquired and reconstructed, reducing scan time while improving signal quality and imaging access.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 3</div>
      <h3>Generative priors with properties required for medicine</h3>
      <p>
        Develop generative models that are anatomically plausible, controllable, pathology-aware, privacy-preserving, and compatible with the reliability requirements of clinical and safety-critical settings.
      </p>
    </div>
  </div>
  <div class="button-row">
    <a class="btn-hero-secondary" href="{{ '/research/' | relative_url }}">Read the full research vision</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Selected work</div>
    <h2>Representative contributions</h2>
    <p>
      Rather than treating publications as a single undifferentiated list, these projects show the core technical ideas shaping my research program.
    </p>
  </div>
  <div class="card-grid">
    <div class="feature-card">
      <div class="tag">Anisotropic MRI</div>
      <h3>Zero-shot and operator-aware super-resolution</h3>
      <p>
        Developed internally trained and cycle-consistent methods for through-plane MRI super-resolution that exploit acquisition operators instead of depending on external paired datasets.
      </p>
      <div class="links">
        <a href="{{ '/projects/' | relative_url }}">Project overview</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div>
    </div>
    <div class="feature-card">
      <div class="tag">Generative priors</div>
      <h3>Physics-grounded generative inference</h3>
      <p>
        Built diffusion-based formulations that enforce exact agreement with acquired data while using generative models as priors for inverse problems in MRI.
      </p>
      <div class="links">
        <a href="{{ '/research/' | relative_url }}">Research context</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div>
    </div>
    <div class="feature-card">
      <div class="tag">3D medical data</div>
      <h3>Large-scale curation and model development</h3>
      <p>
        Helped curate and analyze large 3D brain MRI datasets for diffusion-model training, with emphasis on anatomical plausibility and downstream utility for reconstruction and analysis tasks.
      </p>
      <div class="links">
        <a href="{{ '/projects/' | relative_url }}">Project overview</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div>
    </div>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Recent activity</div>
    <h2>Momentum in publications, talks, and leadership</h2>
  </div>
  <div class="timeline-list">
    <div class="timeline-card">
      <div class="date">2025</div>
      <div>
        <h3>Best Poster Award at IPMI</h3>
        <p>Awarded for <em>Cycle-Consistent Zero-Shot Through-Plane Super-Resolution for Anisotropic Head MRI</em>.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">2025</div>
      <div>
        <h3>Invited talks in Japan</h3>
        <p>Presented invited talks on through-plane super-resolution at the University of Tokyo and the Nara Institute of Science and Technology.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">2024–26</div>
      <div>
        <h3>SASHIMI organizing committee</h3>
        <p>Serving on the organizing committee for SASHIMI at MICCAI across three consecutive years.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">Teaching</div>
      <div>
        <h3>Head TA in core imaging courses at Johns Hopkins</h3>
        <p>Head TA for Medical Image Analysis, Medical Imaging Systems, and Music Signal Processing.</p>
      </div>
    </div>
  </div>
</section>

<section class="section-block">
  <div class="mini-grid">
    <div class="mini-card">
      <h3>Teaching and mentorship</h3>
      <p>
        I care about teaching students to think across theory, imaging physics, implementation, and scientific evaluation. My teaching experience centers on courses that connect signals, systems, and medical image analysis to modern AI practice.
      </p>
      <div class="links">
        <a href="{{ '/teaching/' | relative_url }}">Teaching page</a>
      </div>
    </div>
    <div class="mini-card">
      <h3>Professional service</h3>
      <p>
        My service includes workshop organizing, peer review across major journals and conferences, and visible engagement with the medical imaging community.
      </p>
      <div class="links">
        <a href="{{ '/service/' | relative_url }}">Service page</a>
      </div>
    </div>
  </div>
</section>

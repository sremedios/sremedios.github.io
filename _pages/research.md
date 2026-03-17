---
layout: page
permalink: /research/
title: research
nav: true
nav_order: 1
description: Research vision, current themes, and future lab agenda.
---

<div class="page-anchor-nav">
  <a href="#vision">Vision</a>
  <a href="#foundations">Foundations</a>
  <a href="#thrusts">Future thrusts</a>
  <a href="#impact">Impact</a>
</div>

<section id="vision" class="section-block">
  <div class="section-heading">
    <div class="kicker">Research vision</div>
    <h2>General algorithms for medical imaging that work for every patient, on every scanner, anywhere</h2>
    <p>
      My research asks how AI systems for medical imaging can remain reliable when image appearance changes across scanners, protocols, sites, resolutions, and patient populations. I address this by tightly coupling data-driven models with imaging physics, acquisition operators, anatomical structure, and explicit constraints on inference.
    </p>
  </div>
  <div class="callout-card">
    <h3>Why this direction matters</h3>
    <p>
      Many current medical imaging systems work well only under narrow conditions. By grounding algorithms in image formation and physical constraints, we can build methods that are more robust, interpretable, data-consistent, and clinically trustworthy while also opening new problems in machine learning, optimization, computer vision, and scientific AI.
    </p>
  </div>
</section>

<section id="foundations" class="section-block">
  <div class="section-heading">
    <div class="kicker">Existing foundations</div>
    <h2>From anisotropic MRI super-resolution to 3D generative priors</h2>
    <p>
      My previous work builds a foundation for a broader faculty research program in grounded generative imaging.
    </p>
  </div>
  <div class="card-grid">
    <div class="feature-card">
      <div class="tag">Acquisition-aware learning</div>
      <h3>Operator-grounded super-resolution</h3>
      <p>
        I developed methods such as ECLARE that learn from the data already present in a scan by simulating paired low-resolution observations through an approximate acquisition operator. This removes the need for external paired training sets and better reflects how medical images are actually formed.
      </p>
    </div>
    <div class="feature-card">
      <div class="tag">Stable inverse problems</div>
      <h3>Data consistency and hallucination control</h3>
      <p>
        I developed formulations that embed acquisition operators into constrained reconstruction pipelines, restricting solutions from hallucinating unsupported information and enforcing agreement with measured data.
      </p>
    </div>
    <div class="feature-card">
      <div class="tag">Generative data foundations</div>
      <h3>Large-scale 3D MRI curation and diffusion models</h3>
      <p>
        I have contributed to the curation and analysis of large open brain MRI collections for training generative priors, emphasizing quality control, anatomical plausibility, and downstream usefulness for reconstruction and analysis.
      </p>
    </div>
  </div>
</section>

<section id="thrusts" class="section-block">
  <div class="section-heading">
    <div class="kicker">Future lab agenda</div>
    <h2>Three research thrusts</h2>
    <p>
      Together these directions define a new paradigm of physics-grounded generative image processing and analysis.
    </p>
  </div>
  <div class="thrust-grid">
    <div class="thrust-card">
      <div class="tag">Thrust 1</div>
      <h3>Multi-image and multi-modal computational imaging</h3>
      <p>
        Patients are rarely imaged only once. I plan to formalize multi-image inference as a structured generative inverse problem in which each scan is a partial rendering of the same anatomy. This connects multi-view geometry, inverse problems, and probabilistic modeling in a clinically realistic setting.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 2</div>
      <h3>Co-design of acquisition and reconstruction algorithms</h3>
      <p>
        Current scanners balance time, signal-to-noise ratio, and resolution using classical principles such as Shannon-Nyquist and compressed sensing. I want to extend that design space using differentiable physics and generative priors to learn better sensing and reconstruction strategies together.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 3</div>
      <h3>Generative priors with properties required for medicine</h3>
      <p>
        Medical imaging requires generative models that are not merely expressive, but also anatomically plausible, pathology-aware, privacy-preserving, and theoretically grounded. This thrust develops the reliability theory for generative imaging in medicine and other safety-critical scientific domains.
      </p>
    </div>
  </div>
</section>

<section id="impact" class="section-block">
  <div class="section-heading">
    <div class="kicker">Vision of impact</div>
    <h2>Reliable AI for imaging, with implications beyond healthcare</h2>
    <p>
      The long-term aim is a future where imaging systems are faster, more accessible, and more trustworthy because AI models are grounded in the real acquisition processes that generate data. This has direct relevance to medical imaging, but the underlying ideas also matter for robotics, remote sensing, scientific measurement, and any domain where learned models interact with physical sensing systems.
    </p>
  </div>
  <div class="button-row">
    <a class="btn-hero" href="{{ '/assets/pdf/ResearchStatement.pdf' | relative_url }}">Download research statement</a>
    <a class="btn-hero-secondary" href="{{ '/projects/' | relative_url }}">See project summaries</a>
  </div>
</section>

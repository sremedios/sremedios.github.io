---
layout: page
permalink: /research/
title: research
nav: true
nav_order: 1
description: Research vision, current foundations, and future lab agenda in computational medical imaging.
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
    <h2>Research program in computational medical imaging</h2>
    <p>
      My research develops machine learning methods for medical imaging that remain dependable when image appearance varies across acquisition settings, institutions, spatial resolutions, and patient populations. I address this challenge by combining learned models with physical measurement models, inverse-problem structure, anatomy, and explicit constraints on inference.
    </p>
    <p>
      The broader goal is to move beyond reconstruction systems that work only under narrow conditions and toward methods that hold up in real clinical environments. This motivates a long-term agenda in computational imaging that integrates physical measurement models with modern generative methods to produce systems that are accurate, controllable, and well matched to clinical use.
    </p>
    <!-- <img src="/assets/img/focvs.jpg" 
     alt="Computational imaging research pipeline"
     class="research-figure"> -->
  </div>
  <div class="callout-card">
    <h3>Why this direction matters</h3>
    <p>
      Many high-performing imaging systems still depend on restrictive assumptions about training data, scanner conditions, or acquisition protocols. Building inference around image formation and physical constraints improves robustness and interpretability while opening new questions at the intersection of machine learning, optimization, computer vision, and scientific AI.
    </p>
  </div>
</section>

<section id="foundations" class="section-block">
  <div class="section-heading">
    <div class="kicker">Existing foundations</div>
    <h2>Foundations for an independent research program</h2>
    <p>
      My existing work establishes the scientific and technical foundations for an independent research program in computational imaging. Across these projects, a common theme is that stronger methods come from taking the acquisition process seriously rather than treating imaging as a purely data-driven prediction problem.
    </p>
  </div>
  <div class="card-grid">
    <div class="feature-card">
      <div class="tag">Acquisition-aware learning</div>
      <h3>Operator-grounded super-resolution</h3>
      <p>
        I developed methods such as ECLARE that learn from structure within a scan by simulating paired low-resolution observations through an approximate acquisition operator. This reduces dependence on external paired datasets and aligns learning with the way MRI is actually formed.
      </p>
    </div>
    <div class="feature-card">
      <div class="tag">Stable inverse problems</div>
      <h3>Data consistency and hallucination control</h3>
      <p>
        I developed formulations that embed acquisition operators directly into constrained reconstruction, explicitly enforcing agreement with measured data and reducing the risk of anatomically plausible but unsupported image content.
      </p>
    </div>
    <div class="feature-card">
      <div class="tag">Generative data foundations</div>
      <h3>Large-scale 3D MRI curation and diffusion models</h3>
      <p>
        I developed a large-scale generative foundation model via extensive training, experiments, and especially curation and analysis of large 3D brain MRI datasets. I emphasized quality control, anatomical plausibility, and downstream value for reconstruction and analysis tasks.
      </p>
    </div>
  </div>
</section>

<section id="thrusts" class="section-block">
  <div class="section-heading">
    <div class="kicker">Future lab agenda</div>
    <h2>Three research thrusts for my future lab</h2>
    <p>
      Together, these directions define a future lab agenda that is technically ambitious, clinically grounded, and broad enough to support sustained methodological leadership.
    </p>
  </div>
  <div class="thrust-grid">
    <div class="thrust-card">
      <div class="tag">Thrust 1</div>
      <h3>Multi-image and multi-modal computational imaging</h3>
      <p>
        Formalize multi-image inference as a structured generative inverse problem in which each scan is a partial observation of shared anatomy. This direction connects multi-view geometry, probabilistic modeling, and inverse problems in clinically realistic settings involving multiple contrasts, visits, sites, and modalities.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 2</div>
      <h3>Co-design of acquisition and reconstruction</h3>
      <p>
        Co-design sensing and reconstruction with differentiable forward models and learned priors to improve scan efficiency, signal quality, and accessibility. The aim is not only better reconstruction, but also a tighter integration between how images are acquired and how information is inferred from them.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 3</div>
      <h3>Generative priors for medicine</h3>
      <p>
        Develop generative models with properties required for medicine: anatomical plausibility, controllability, pathology awareness, privacy preservation, and reliability under deployment constraints. A central objective is to make generative priors usable in clinical imaging without sacrificing safety or data fidelity.
      </p>
    </div>
  </div>
</section>

<section id="impact" class="section-block">
  <div class="section-heading">
    <div class="kicker">Vision of impact</div>
    <h2>Trustworthy imaging methods, with implications beyond healthcare</h2>
    <p>
      The long-term goal of this research is medical imaging that is faster, more accessible, and more trustworthy because inference is tied to the measurement process rather than statistical pattern matching alone. While healthcare is the primary domain, the underlying ideas extend to robotics, remote sensing, scientific measurement, and other settings in which learned models must interact responsibly with physical sensors.
    </p>
  </div>
  <div class="button-row">
    <a class="btn-hero" href="{{ '/assets/pdf/ResearchStatement.pdf' | relative_url }}">Download research statement</a>
    <a class="btn-hero-secondary" href="{{ '/projects/' | relative_url }}">See project summaries</a>
  </div>
</section>

---
layout: home
title: Samuel W. Remedios
permalink: /
description: Physics-grounded AI for reliable medical imaging.
nav: false
---

<section class="home-hero">
  <div class="eyebrow">Faculty candidate in computational medical imaging</div>
  <div class="hero-grid">
    <div>
      <h1>Physics-grounded AI for reliable medical imaging</h1>
      <p class="hero-lede">
        I develop machine learning methods for medical imaging that explicitly model acquisition physics, anatomy, and data consistency. My goal is to build algorithms that remain reliable across scanners, protocols, and patient populations, and to lay the foundations for a broader research program in generative computational imaging.
      </p>
      <div class="hero-meta">
        <span class="meta-pill">Postdoctoral Researcher, Johns Hopkins University</span>
        <span class="meta-pill">Medical imaging</span>
        <span class="meta-pill">Generative modeling</span>
        <span class="meta-pill">Inverse problems</span>
        <span class="meta-pill">Computational imaging</span>
      </div>
      <div class="hero-actions">
        <a class="btn-hero" href="{{ '/research/' | relative_url }}">Research agenda</a>
        <a class="btn-hero-secondary" href="{{ '/projects/' | relative_url }}">Selected contributions</a>
        <a class="btn-hero-secondary" href="{{ '/publications/' | relative_url }}">Publications</a>
        <a class="btn-hero-secondary" href="{{ '/assets/pdf/CurriculumVitae.pdf' | relative_url }}">Download CV</a>
      </div>
    </div>

    <aside class="hero-card">
      <img src="/assets/img/headshot.jpg" alt="Samuel W. Remedios" class="headshot">
      <h3>At a glance</h3>
      <ul>
        <li>Builds MRI reconstruction and generative imaging methods grounded in acquisition physics</li>
        <li>Focuses on robustness across scanners, protocols, and patient populations</li>
        <li>Develops a research program spanning inverse problems, super-resolution, and generative priors</li>
        <li>Combines technical depth with visible momentum, awards, and field service</li>
      </ul>
      <h3>Selected distinctions</h3>
      <div class="inline-stat-list">
        <span class="inline-stat">NSF GRFP</span>
        <span class="inline-stat">IPMI 2025 Best Poster</span>
        <span class="inline-stat">SASHIMI 2023 Best Paper</span>
        <span class="inline-stat">Invited talks: Tokyo, NAIST</span>
      </div>
    </aside>
  </div>
</section>

<section class="home-section">
  <div class="section-kicker">Research identity</div>
  <h2>A research program for reliable medical imaging</h2>
  <p class="section-intro">
    I work at the intersection of machine learning, optimization, and medical imaging. Across my work, I ask how we can move beyond purely data-driven reconstruction toward methods that incorporate imaging physics, leverage structure in anatomy and acquisition, and remain dependable in real clinical settings.
  </p>

  <div class="glance-grid">
    <div class="glance-card">
      <div class="glance-year">Core problem</div>
      <h3>Reliability across clinical variation</h3>
      <p>Design imaging algorithms that continue to perform across heterogeneous scanners, acquisition protocols, and patient populations.</p>
    </div>
    <div class="glance-card">
      <div class="glance-year">Technical lens</div>
      <h3>Physics-grounded inference</h3>
      <p>Develop inverse-problem and generative methods that enforce agreement with measured data instead of depending only on paired training sets.</p>
    </div>
    <div class="glance-card">
      <div class="glance-year">Application domain</div>
      <h3>MRI and computational imaging</h3>
      <p>Focus on MRI as a core domain for reconstruction, super-resolution, and generative modeling, with attention to the practical constraints of clinical imaging.</p>
    </div>
    <div class="glance-card">
      <div class="glance-year">Long-term vision</div>
      <h3>Generative computational imaging</h3>
      <p>Build a broader program that co-designs imaging systems and inference algorithms for robust, efficient, and accessible medical imaging.</p>
    </div>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Research agenda</div>
    <h2>Three research thrusts</h2>
    <p>
      My work centers on combining imaging physics with modern generative modeling to create methods that are not only accurate, but also structured, controllable, and clinically reliable.
    </p>
  </div>
  <div class="thrust-grid">
    <div class="thrust-card">
      <div class="tag">Thrust 1</div>
      <h3>Multi-image and multi-modal computational imaging</h3>
      <p>
        Develop structured generative frameworks that integrate scans across visits, contrasts, sites, and modalities for reconstruction, longitudinal analysis, and downstream clinical tasks.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 2</div>
      <h3>Co-design of acquisition and reconstruction</h3>
      <p>
        Couple acquisition design with reconstruction through differentiable physics and learned priors to reduce scan time while improving image quality and access.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Thrust 3</div>
      <h3>Generative priors for medicine</h3>
      <p>
        Build generative models that are anatomically plausible, controllable, pathology-aware, privacy-preserving, and aligned with the requirements of clinical deployment.
      </p>
    </div>
  </div>
  <div class="button-row">
    <a class="btn-hero-secondary" href="{{ '/research/' | relative_url }}">Read the full research vision</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Selected contributions</div>
    <h2>Representative projects and technical contributions</h2>
    <p>
      These projects show how my research combines physical modeling, generative methods, and clinically motivated problem design.
    </p>
  </div>
  <div class="card-grid">
    <div class="feature-card">
      <div class="tag">MRI super-resolution</div>
      <h3>Zero-shot and operator-aware through-plane super-resolution</h3>
      <p>
        Developed internally trained and cycle-consistent methods for anisotropic MRI super-resolution that use acquisition operators directly, reducing dependence on paired high-resolution training data.
      </p>
      <!-- <div class="links">
        <a href="{{ '/projects/' | relative_url }}">Project overview</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div> -->
    </div>
    <div class="feature-card">
      <div class="tag">Inverse problems</div>
      <h3>Generative priors with data-consistent inference</h3>
      <p>
        Developed diffusion-based formulations for MRI inverse problems that use generative models as priors while explicitly enforcing consistency with measured observations.
      </p>
      <!-- <div class="links">
        <a href="{{ '/research/' | relative_url }}">Research context</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div> -->
    </div>
    <div class="feature-card">
      <div class="tag">3D brain MRI</div>
      <h3>Large-scale data curation for generative imaging</h3>
      <p>
        Curated and analyzed large-scale 3D brain MRI datasets for diffusion-model development, with emphasis on anatomical plausibility and downstream utility for reconstruction tasks.
      </p>
      <!-- <div class="links">
        <a href="{{ '/projects/' | relative_url }}">Project overview</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div> -->
    </div>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Recognition and professional activity</div>
    <h2>Recent honors, invited talks, and service</h2>
  </div>
  <div class="timeline-list">
    <div class="timeline-card">
      <div class="date">2025</div>
      <div>
        <h3>IPMI 2025 Best Poster Award</h3>
        <p>Awarded for <em>Cycle-Consistent Zero-Shot Through-Plane Super-Resolution for Anisotropic Head MRI</em>.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">2025</div>
      <div>
        <h3>Invited talks in Japan</h3>
        <p>Presented invited talks on through-plane MRI super-resolution at the University of Tokyo and the Nara Institute of Science and Technology.</p>
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
      <div class="date">2026</div>
      <div>
        <h3>SynthOCT organizing committee</h3>
        <p>Serving on the organizing committee for the SynthOCT 2026 challenge at MICCAI.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">2020–25</div>
      <div>
        <h3>NSF Graduate Research Fellowship</h3>
        <p>Supported independent research in AI for medical imaging through the National Science Foundation Graduate Research Fellowship Program.</p>
      </div>
    </div>
  </div>
</section>

<section class="section-block">
  <div class="mini-grid">
    <div class="mini-card">
      <h3>Teaching and mentorship</h3>
      <p>
        I teach students to connect imaging physics, algorithms, and modern AI, with emphasis on conceptual understanding, technical rigor, and critical evaluation of model behavior.
      </p>
      <div class="links">
        <a href="{{ '/teaching/' | relative_url }}">Teaching page</a>
      </div>
    </div>
    <div class="mini-card">
      <h3>Professional service</h3>
      <p>
        My service includes workshop organization, peer review for leading venues, and sustained engagement with the medical imaging research community.
      </p>
      <div class="links">
        <a href="{{ '/service/' | relative_url }}">Service page</a>
      </div>
    </div>
  </div>
</section>

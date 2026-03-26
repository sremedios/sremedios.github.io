// get the ninja-keys element
const ninja = document.querySelector('ninja-keys');

// add the home and posts menu items
ninja.data = [{
    id: "nav-samuel-w-remedios",
    title: "Samuel W. Remedios",
    section: "Navigation",
    handler: () => {
      window.location.href = "/";
    },
  },{id: "nav-research",
          title: "research",
          description: "Research vision, current foundations, and future lab agenda in computational medical imaging.",
          section: "Navigation",
          handler: () => {
            window.location.href = "/research/";
          },
        },{id: "nav-publications",
          title: "publications",
          description: "Selected publications, full bibliography, and a record of work in medical imaging, inverse problems, and generative methods.",
          section: "Navigation",
          handler: () => {
            window.location.href = "/publications/";
          },
        },{id: "nav-cv",
          title: "cv",
          description: "Full curriculum vitae, including publications, talks, teaching, service, and awards.",
          section: "Navigation",
          handler: () => {
            window.location.href = "/cv/";
          },
        },{id: "books-the-godfather",
          title: 'The Godfather',
          description: "",
          section: "Books",handler: () => {
              window.location.href = "/books/the_godfather.html";
            },},{id: "news-a-simple-inline-announcement",
          title: 'A simple inline announcement.',
          description: "",
          section: "News",},{id: "news-a-long-announcement-with-details",
          title: 'A long announcement with details',
          description: "",
          section: "News",handler: () => {
              window.location.href = "/news/announcement_2.html";
            },},{id: "news-a-simple-inline-announcement-with-markdown-emoji-sparkles-smile",
          title: 'A simple inline announcement with Markdown emoji! :sparkles: :smile:',
          description: "",
          section: "News",},{id: "news-received-the-best-poster-award-at-ipmi-2025-for-work-on-cycle-consistent-zero-shot-through-plane-super-resolution-of-anisotropic-head-mri",
          title: 'Received the Best Poster Award at IPMI 2025 for work on cycle-consistent zero-shot...',
          description: "",
          section: "News",},{id: "news-gave-an-invited-talk-at-the-university-of-tokyo-on-through-plane-super-resolution-of-anisotropic-multi-slice-mri",
          title: 'Gave an invited talk at The University of Tokyo on through-plane super-resolution of...',
          description: "",
          section: "News",},{id: "news-presented-an-invited-talk-at-the-nara-institute-of-science-and-technology-on-through-plane-super-resolution-of-anisotropic-multi-slice-mri",
          title: 'Presented an invited talk at The Nara Institute of Science and Technology on...',
          description: "",
          section: "News",},{
        id: 'social-cv',
        title: 'CV',
        section: 'Socials',
        handler: () => {
          window.open("/assets/pdf/CirriculumVitae.pdf", "_blank");
        },
      },{
        id: 'social-email',
        title: 'email',
        section: 'Socials',
        handler: () => {
          window.open("mailto:%73%61%6D%75%65%6C.%72%65%6D%65%64%69%6F%73@%6A%68%75.%65%64%75", "_blank");
        },
      },{
        id: 'social-github',
        title: 'GitHub',
        section: 'Socials',
        handler: () => {
          window.open("https://github.com/sremedios", "_blank");
        },
      },{
        id: 'social-scholar',
        title: 'Google Scholar',
        section: 'Socials',
        handler: () => {
          window.open("https://scholar.google.com/citations?user=H6TdLnoAAAAJ", "_blank");
        },
      },{
        id: 'social-x',
        title: 'X',
        section: 'Socials',
        handler: () => {
          window.open("https://twitter.com/sam_remedios", "_blank");
        },
      },];

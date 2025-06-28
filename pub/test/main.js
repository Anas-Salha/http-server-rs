// main.js — continuously updates the clock + form feedback

// run when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  console.log("✅ External JavaScript loaded successfully!");

  /* --------  live clock  -------- */
  const clockEl = document.getElementById("clock");

  // helper to pad zeros
  const pad = (n) => n.toString().padStart(2, "0");

  function updateClock() {
    const now = new Date();
    const h = pad(now.getHours());
    const m = pad(now.getMinutes());
    const s = pad(now.getSeconds());
    clockEl.textContent = `${h}:${m}:${s}`;
  }

  updateClock(); // set immediately
  setInterval(updateClock, 1000); // update every second

  /* --------  contact-form feedback  -------- */
  const form = document.getElementById("contactForm");
  if (form) {
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      alert("Thanks for reaching out!");
      form.reset();
    });
  }
});

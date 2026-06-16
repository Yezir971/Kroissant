document.body.addEventListener('htmx:beforeRequest', function(evt) {
  const trigger = evt.detail.elt;
  const container = document.getElementById('registration-container');
  if (!container) return;
  if (trigger && trigger.dataset.direction === 'back') {
    container.classList.add('slide-back');
  } else {
    container.classList.remove('slide-back');
  }
});

document.body.addEventListener('htmx:afterSettle', function() {
  const container = document.getElementById('registration-container');
  if (container) container.classList.remove('slide-back');
});

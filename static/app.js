console.log('App.js chargé');

document.addEventListener('DOMContentLoaded', function() {
  document.body.addEventListener('htmx:beforeRequest', function(evt) {
    console.log('HTMX Request:', evt.detail.path);
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
});

function togglePassword(inputId, iconId) {
  const input = document.getElementById(inputId);
  const icon = document.getElementById(iconId);
  if (!input || !icon) return;
  if (input.type === 'password') {
    input.type = 'text';
    icon.src = '/static/img/oeil-dash.svg';
  } else {
    input.type = 'password';
    icon.src = '/static/img/oeil.svg';
  }
}

function validatePasswords() {
  const p1 = document.getElementById('password');
  const p2 = document.getElementById('password_confirm');
  const btn = document.getElementById('submit-btn');
  if (!p1 || !p2 || !btn) return;
  
  if (p1.value.length >= 8 && p1.value === p2.value) {
    btn.disabled = false;
  } else {
    btn.disabled = true;
  }
}

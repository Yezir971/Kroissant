const CACHE_NAME = 'kroissant-v18';
const ASSETS_TO_CACHE = [
  '/static/app.css',
  '/static/app.js',
  '/static/htmx.min.js',
  '/static/img/blue_desktop.webp',
  '/static/img/bluey_mobile.webp',
  '/static/img/hilda_desktop.webp',
  '/static/img/hilda_mobile.webp',
  '/static/img/puffin_rock_desktop.webp',
  '/static/img/puffin_rock_mobile.webp',
  '/static/img/ploopy.svg'
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(ASSETS_TO_CACHE);
    })
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (cacheName !== CACHE_NAME) {
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
});

self.addEventListener('fetch', (event) => {
  if (event.request.mode === 'navigate') {
    event.respondWith(fetch(event.request));
    return;
  }

  event.respondWith(
    caches.match(event.request).then((response) => {
      return response || fetch(event.request);
    })
  );
});

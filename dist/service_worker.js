self.addEventListener('install', (event) => {
    console.log('Service Worker installing.');
    event.waitUntil(
        caches.open('v1').then((cache) => {
            return cache.addAll([
                '/',
                '/index.html',
                '/egui_website.js',
                '/egui_website_bg.wasm',
                '/style.css',
            ]);
        })
    );
});

self.addEventListener('fetch', (event) => {
    console.log('Service Worker fetching:', event.request.url);
    if (event.request.url.endsWith('.wasm')) {
        event.respondWith(
            fetch(event.request).then((response) => {
                if (response.ok) {
                    const newHeaders = new Headers(response.headers);
                    newHeaders.set('Content-Type', 'application/wasm');
                    return new Response(response.body, {
                        status: response.status,
                        statusText: response.statusText,
                        headers: newHeaders,
                    });
                }
                return response;
            })
        );
    } else {
        event.respondWith(
            caches.match(event.request).then((response) => {
                return response || fetch(event.request);
            })
        );
    }
});

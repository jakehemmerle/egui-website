self.addEventListener('fetch', event => {
  if (event.request.url.endsWith('.wasm')) {
    console.log('Service worker intercepting fetch request for .wasm file:', event.request.url);
    event.respondWith(
      fetch(event.request).then(response => {
        const newHeaders = new Headers(response.headers);
        newHeaders.set('Content-Type', 'application/wasm');
        const modResponse = new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers: newHeaders
        });
        console.log('Service worker responding with modified headers for .wasm file:', event.request.url);
        return modResponse;
      }).catch(error => {
        console.error('Service worker failed to fetch .wasm file:', event.request.url, error);
        throw error;
      })
    );
  }
});

self.addEventListener('fetch', event => {
  if (event.request.url.endsWith('.wasm')) {
    event.respondWith(
      fetch(event.request).then(response => {
        const newHeaders = new Headers(response.headers);
        newHeaders.set('Content-Type', 'application/wasm');
        const modResponse = new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers: newHeaders
        });
        return modResponse;
      })
    );
  }
});

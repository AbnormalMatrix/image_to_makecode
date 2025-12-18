import http.server
import socketserver

PORT = 8000
Handler = http.server.SimpleHTTPRequestHandler
# Explicitly map the .wasm extension
Handler.extensions_map.update({
    '.wasm': 'application/wasm',
})

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"Serving WASM at http://localhost:{PORT}")
    httpd.serve_forever()
#!/usr/bin/env python3
"""
Simple HTTP server for serving the WebAssembly spiral application
with proper CORS headers and MIME types.
"""

import http.server
import socketserver
import os
from pathlib import Path

class WasmHandler(http.server.SimpleHTTPRequestHandler):
    """Custom handler to serve WebAssembly files with correct MIME types"""
    
    def end_headers(self):
        # Add CORS headers
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()
    
    def guess_type(self, path):
        """Override to set correct MIME types for WebAssembly files"""
        if path.endswith('.wasm'):
            return 'application/wasm'
        elif path.endswith('.js'):
            return 'application/javascript'
        elif path.endswith('.html'):
            return 'text/html'
        elif path.endswith('.css'):
            return 'text/css'
        else:
            # Use the parent implementation for other files
            return super().guess_type(path)

def main():
    PORT = 8000
    
    # Change to the project root directory to serve both web/ and pkg/
    project_root = Path(__file__).parent.parent
    os.chdir(project_root)
    
    print(f"Serving from directory: {os.getcwd()}")
    print(f"Available files in web/: {list(Path('web').glob('*'))}")
    print(f"Available files in pkg/: {list(Path('pkg').glob('*')) if Path('pkg').exists() else 'pkg/ not found'}")
    
    with socketserver.TCPServer(("", PORT), WasmHandler) as httpd:
        print(f"Serving WebAssembly spiral application at http://localhost:{PORT}")
        print(f"Open http://localhost:{PORT}/web/ to view the application")
        print("Press Ctrl+C to stop the server")
        
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped.")

if __name__ == "__main__":
    main()
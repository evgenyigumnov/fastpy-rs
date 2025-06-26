import pytest
import fastpy_rs
import time
import requests
from http.server import HTTPServer, BaseHTTPRequestHandler
# Configuration
SERVER_HOST = '127.0.0.1'
SERVER_PORT = 8080
BASE_URL = f'http://{SERVER_HOST}:{SERVER_PORT}/'
NUM_REQUESTS = 10  # Number of requests for the benchmark
CONCURRENT_REQUESTS = 10  # Number of concurrent requests

import threading

response_text = "Hello, World!"


class HelloHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-Type", "text/plain; charset=utf-8")
        self.send_header("Content-Length", str(len(response_text.encode("utf-8"))))
        self.end_headers()
        self.wfile.write(response_text.encode("utf-8"))

def run_server():
    httpd = HTTPServer((SERVER_HOST, SERVER_PORT), HelloHandler)
    print(f"Serving at http://{SERVER_HOST}:{SERVER_PORT}")
    httpd.serve_forever()

server_thread = threading.Thread(target=run_server, daemon=True)
server_thread.start()
# sleep 1 sec
time.sleep(1)



def test_http_get_python(benchmark):
    """Benchmark the Python requests library for comparison."""
    # Warm-up the server
    requests.get(BASE_URL).raise_for_status()

    # The actual benchmark
    def make_request():
        response = requests.get(BASE_URL)
        response.raise_for_status()
        assert 'Hello, World!' in response.text

    benchmark.pedantic(make_request, rounds=NUM_REQUESTS, iterations=CONCURRENT_REQUESTS)



def test_http_get_rust(benchmark):
    """Benchmark the Rust implementation of http_get."""
    #Warm-up the server
    fastpy_rs.http.get(f"{BASE_URL}")

    # The actual benchmark
    def make_request():
        result = fastpy_rs.http.get(f"{BASE_URL}")
        assert 'Hello, World!' in result

    benchmark.pedantic(make_request, rounds=NUM_REQUESTS, iterations=CONCURRENT_REQUESTS)


if __name__ == "__main__":
    try:
        # Run the benchmark
        import pytest
        pytest.main([__file__, '-v', '--benchmark-only'])
    finally:
        # Ensure the server is stopped when done
        try:
            requests.get(f"http://{SERVER_HOST}:{SERVER_PORT}/shutdown", timeout=1)
        except:
            pass

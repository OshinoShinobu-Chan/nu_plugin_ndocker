from http.server import HTTPServer, BaseHTTPRequestHandler
import os

class CustomRequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        # 只处理特定路径的请求
        if self.path == '/image/openjdk.tar':
            print("Received request for /image/openjdk.tar")
            file_path = '/root/openjdk.tar'  # 文件路径
            
            # 检查文件是否存在
            if not os.path.exists(file_path):
                self.send_error(404, "File Not Found")
                return
            
            try:
                # 设置响应头
                self.send_response(200)
                self.send_header('Content-type', 'application/x-tar')
                self.send_header('Content-Disposition', f'attachment; filename="{os.path.basename(file_path)}"')
                self.end_headers()
                
                # 以二进制模式读取文件并发送
                with open(file_path, 'rb') as f:
                    self.wfile.write(f.read())
            except Exception as e:
                self.send_error(500, f"Server Error: {str(e)}")
        else:
            self.send_error(404, "Path Not Found")

def run_server(port=12345):
    server_address = ('', port)  # 监听所有可用IP
    httpd = HTTPServer(server_address, CustomRequestHandler)
    print(f"Server is up, listening {port}...")
    httpd.serve_forever()

if __name__ == '__main__':
    run_server()
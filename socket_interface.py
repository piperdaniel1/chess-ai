import socket
import time

class Connection:
    def __init__(self):
        self.refresh_socket()
        self.last_received = 0
        self.received_last = True
        self.queue = []
    
    def refresh_socket(self):
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
    def handle_queue(self):
        if self.received_last and len(self.queue) > 0:
            self.__send(message=self.queue.pop(0))
        
    def push_to_queue(self, message):
        self.queue.append(message)

    def __send(self, ip='127.0.0.1', port=4321, message='ping'):
        print(f"Sending message: '{message}'")

        try:
            self.socket.connect((ip, port))
            self.socket.setblocking(False)
        except OSError as e:
            if e.errno == 106:
                pass
            else:
                raise e

        self.socket.sendall(message.encode('utf-8'))
        self.received_last = False

    def receive(self):
        if time.time() - self.last_received < 0.1 or self.received_last == True:
            return False, None
        
        self.last_received = time.time()
        status = True
        try:
            val = self.socket.recv(1024)
            if val == b'':
                print("received empty string")
                status = False
        except socket.timeout:
            print("timeout")
            val = None
            status = False
        except socket.error as e:
            if e.errno != 11:
                print("error:", e)

            val = -1
            status = False
        
        if status:
            assert(val != None and val != -1)
            print(f"Received message: '{val.decode('utf-8')}'")

            self.received_last = True
            self.socket.close()
            self.refresh_socket()

        return status, val

    def close(self):
        self.socket.close()

# interactive mode
if __name__ == "__main__":
    conn = Connection()

    while True:
        message = input("Enter message: ")
        if message == "exit":
            break
        
        if message != "":
            conn.push_to_queue(message)
        
        conn.handle_queue()
        status, val = conn.receive()

        if status:
            assert(val != None and val != -1)
            print("Received message:", val.decode('utf-8'))

        time.sleep(0.1)

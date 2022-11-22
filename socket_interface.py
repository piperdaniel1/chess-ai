import socket

def send_message(message):
    # send a socket message to port 4321
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # connect to the socket
    s.connect(('localhost', 4321))

    # send a message
    s.send(message.encode('utf-8'))

    # receive a message
    data = s.recv(1024)

    # close the socket
    s.close()

    data = data.decode('utf-8')

    return data

while True:
    message = input()
    if message == 'exit':
        break
    response = send_message(message)

    print(" >", response)

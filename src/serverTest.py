import socket
import numpy as np
import PIL.Image as Image
#import pnwcybersec

HOST = 'localhost'
PORT = 3333
MAX = 2000000 # 2MB

def arr2Image(data):

    if len(data) > MAX:
        print("ABORT: file to large")
        return False

    a = bytearray(data)

    width = int(len(data)**0.5)
    g = np.reshape(a[:width * width], (width, width))

    img = Image.fromarray(g)
    img.save('test.png')
    print('Files converted successfully')
    return True

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    conn, addr = s.accept()
    with conn:
        print('Connection by', addr)
        while True:
            data = conn.recv(MAX)
            if not data:
                break
            dataArr = list(data)
            arr2Image(dataArr)
            conn.sendall(b'File recived')
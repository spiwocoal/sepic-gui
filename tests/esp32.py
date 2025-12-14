import socket
import time

UDP_IP = "esp32-pelele.local" # IP del ESP32
UDP_PORT = 4444

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.settimeout(1.0) # Timeout 

try:
    #Enviar señal de inicio
    print("Enviando START...")
    sock.sendto(b"START", (UDP_IP, UDP_PORT))
    last_packet_time = time.time()
    #Recibir datos indefinidamente
    print("Recibiendo datos (Ctrl+C para parar)...")
    while True:
        try:
            data, addr = sock.recvfrom(1024)
            print(f"Dato: {data.decode().strip()}")
            last_packet_time = time.time()
        except socket.timeout:
            time_since_last = time.time() - last_packet_time
                
            if time_since_last > 3.0:
                    
                print(f"Sin datos por {time_since_last:.1f}s. Reenviando START...")
                    
                    
                #Reenviar START
                try:
                        sock.sendto(b"START", (UDP_IP, UDP_PORT))
                except:
                    pass # Si falla el envio
                    # esperamos otro ciclo de timeout.
                last_packet_time = time.time()

except KeyboardInterrupt:
    print("\nUsuario solicitó salir.")

finally:
    #error o sea salida manual
    print("Enviando señal END al ESP32...")
    sock.sendto(b"END", (UDP_IP, UDP_PORT))
    sock.close()
    print("Conexión cerrada.")
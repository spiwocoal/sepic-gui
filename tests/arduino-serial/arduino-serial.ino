unsigned char MAGIC_HEADER[] = {0x06};
char data[20];

void setup() {
  Serial.begin(9600);
}

void loop() {
  if (Serial.available() > 0) {
    int bytesRead = Serial.readBytesUntil('\x03', data, sizeof(data) - 1);
    data[bytesRead] = '\0';
    
    if (!strcmp(data, "\x02\x05")) {
      Serial.write(MAGIC_HEADER, sizeof(MAGIC_HEADER));
    }
  }
}

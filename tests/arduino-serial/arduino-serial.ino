unsigned char MAGIC_HEADER[] = {0x75, 0xE0, 0x5E, 0xB1, 0xC0, 0x00};
char data[20];

void setup() {
  Serial.begin(9600);
}

void loop() {
  if (Serial.available() > 0) {
    int bytesRead = Serial.readBytesUntil('\n', data, sizeof(data) - 1);
    data[bytesRead] = '\0';

    if (strcmp(data, "hello") == 0) {
      Serial.write(MAGIC_HEADER, sizeof(MAGIC_HEADER));
    }
  }
}

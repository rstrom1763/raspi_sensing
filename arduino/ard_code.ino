#include <WiFiNINA.h>
#include <ArduinoHttpClient.h>
#include <stdio.h>

// Replace with your network credentials
const char* ssid = "";
const char* password = "";
WiFiClient wifi;

void setup() {
  // Initialize serial communication
  Serial.begin(9600);
  delay(1000);

  // Connect to Wi-Fi
  Serial.println("Connecting to Wi-Fi...");
  WiFi.begin(ssid, password);

  // Wait until connected
  while (WiFi.status() != WL_CONNECTED) {
    Serial.println("Waiting to connect to wifi");
    delay(500);
    WiFi.begin(ssid, password);
  }

  Serial.println("\nConnected to Wi-Fi!");
  Serial.print("IP Address: ");
  Serial.println(WiFi.localIP());
  
}

void loop() {

  while (WiFi.status() != WL_CONNECTED) {
    Serial.println("Reconnecting to Wi-Fi...");
    WiFi.begin(ssid, password);
    delay(1000);
  }

  makePostRequest(&wifi);

  delay(15000);

}

int makePostRequest(WiFiClient* wifi) {
  
  char name[] = "arduino";
  char serverAddress[] = "";
  int port = ;
  
  HttpClient http = HttpClient(*wifi, serverAddress, port);


  float temp = takeReading();

  // Define the JSON payload
  String jsonPayload = createJson(temp,name);
  Serial.println(jsonPayload);

  // Send the POST request
  int httpResponseCode = http.post("/posttemp","application/json",jsonPayload);
  //Serial.println(httpResponseCode);

  // End the HTTP connection
  http.stop();

}


float takeReading() {

 int sensorPin = A1;
  //getting the voltage reading from the temperature sensor
 float reading = analogRead(sensorPin);  
 
 // converting that reading to voltage, for 3.3v arduino use 3.3
 float voltage = reading * (3.3 / 1024);
  
 // now print out the temperature
 float temperatureC = (voltage - 0.5) * 100 ;  //converting from 10 mv per degree wit 500 mV offset
//to degrees ((voltage - 500mV) times 100)
 
 // now convert to Fahrenheit
 float temperatureF = (temperatureC * 9.0 / 5.0) + 32.0;
 
 return temperatureF;

}

String createJson(float temp, char* name) {

  char buffer[300];

  sprintf(buffer, "{\"name\": \"%s\",\"temp\":%.2f,\"humidity\":0,\"pressure\":0}",name,temp);

  return buffer;

}




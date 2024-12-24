from sense_hat import SenseHat
import requests
import json
import time

config_file = open("./config.json",'r')
config = json.load(config_file)
auth_code = config['auth_code']  # Placeholder for when tokens are implemented
server = config['url']  # The api url

sense = SenseHat()

while True:

    # Get the temp
    temp_celcius = sense.get_temperature()

    # Convert temp to Farenheit
    temp_farenheit = round((temp_celcius * 9/5) + 32, 2)

    # Get air pressure
    pressure = round(sense.get_pressure(), 2)

    # Get humidity
    humidity = round(sense.get_humidity(), 2)

    # Headers for the post request
    headers = {
        'Content-Type': 'application/json'
    }

    #The data points put into a dictionary to be used in the request
    data = {
        'name':config['name'],
        'temp': temp_farenheit,
        'humidity': humidity,
        'pressure': pressure
    }

    try:
        # Send post request to the server
        requests.post(server, data=json.dumps(data), headers=headers, verify=False)
    except:
        print("There was an error")
        continue

    time.sleep(int(config['interval']))  # Wait the specified time before capturing another datapoint